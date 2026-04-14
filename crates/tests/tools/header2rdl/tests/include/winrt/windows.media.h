
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
#ifndef __windows2Emedia_h__
#define __windows2Emedia_h__
#ifndef __windows2Emedia_p_h__
#define __windows2Emedia_p_h__


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

#if !defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)
#define WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.AppService.h"
#include "Windows.Graphics.DirectX.h"
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CIAudioBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IAudioBuffer;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIAudioBuffer ABI::Windows::Media::IAudioBuffer

#endif // ____x_ABI_CWindows_CMedia_CIAudioBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IAudioFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIAudioFrame ABI::Windows::Media::IAudioFrame

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IAudioFrameFactory;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory ABI::Windows::Media::IAudioFrameFactory

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IAutoRepeatModeChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs ABI::Windows::Media::IAutoRepeatModeChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IImageDisplayProperties;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties ABI::Windows::Media::IImageDisplayProperties

#endif // ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaControl_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaControl;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaControl ABI::Windows::Media::IMediaControl

#endif // ____x_ABI_CWindows_CMedia_CIMediaControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaExtension;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaExtension ABI::Windows::Media::IMediaExtension

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaExtensionManager;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager ABI::Windows::Media::IMediaExtensionManager

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaExtensionManager2;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2 ABI::Windows::Media::IMediaExtensionManager2

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaFrame ABI::Windows::Media::IMediaFrame

#endif // ____x_ABI_CWindows_CMedia_CIMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaMarker;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaMarker ABI::Windows::Media::IMediaMarker

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaMarkerTypesStatics;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics ABI::Windows::Media::IMediaMarkerTypesStatics

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarkers_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarkers_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaMarkers;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaMarkers ABI::Windows::Media::IMediaMarkers

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarkers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaProcessingTriggerDetails;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails ABI::Windows::Media::IMediaProcessingTriggerDetails

#endif // ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaTimelineController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaTimelineController;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaTimelineController ABI::Windows::Media::IMediaTimelineController

#endif // ____x_ABI_CWindows_CMedia_CIMediaTimelineController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaTimelineController2;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2 ABI::Windows::Media::IMediaTimelineController2

#endif // ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMediaTimelineControllerFailedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs ABI::Windows::Media::IMediaTimelineControllerFailedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMusicDisplayProperties;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties ABI::Windows::Media::IMusicDisplayProperties

#endif // ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMusicDisplayProperties2;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2 ABI::Windows::Media::IMusicDisplayProperties2

#endif // ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IMusicDisplayProperties3;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3 ABI::Windows::Media::IMusicDisplayProperties3

#endif // ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IPlaybackPositionChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs ABI::Windows::Media::IPlaybackPositionChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IPlaybackRateChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs ABI::Windows::Media::IPlaybackRateChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IShuffleEnabledChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs ABI::Windows::Media::IShuffleEnabledChangeRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControls;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls ABI::Windows::Media::ISystemMediaTransportControls

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControls2;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2 ABI::Windows::Media::ISystemMediaTransportControls2

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControlsButtonPressedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs ABI::Windows::Media::ISystemMediaTransportControlsButtonPressedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControlsDisplayUpdater;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater ABI::Windows::Media::ISystemMediaTransportControlsDisplayUpdater

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControlsPropertyChangedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs ABI::Windows::Media::ISystemMediaTransportControlsPropertyChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControlsStatics;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics ABI::Windows::Media::ISystemMediaTransportControlsStatics

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface ISystemMediaTransportControlsTimelineProperties;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties ABI::Windows::Media::ISystemMediaTransportControlsTimelineProperties

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoDisplayProperties;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties ABI::Windows::Media::IVideoDisplayProperties

#endif // ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoDisplayProperties2;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2 ABI::Windows::Media::IVideoDisplayProperties2

#endif // ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoEffectsStatics;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics ABI::Windows::Media::IVideoEffectsStatics

#endif // ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrame ABI::Windows::Media::IVideoFrame

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrame2;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrame2 ABI::Windows::Media::IVideoFrame2

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrameFactory;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory ABI::Windows::Media::IVideoFrameFactory

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            interface IVideoFrameStatics;
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics ABI::Windows::Media::IVideoFrameStatics

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions

#ifndef DEF___FIAsyncOperation_1_boolean_USE
#define DEF___FIAsyncOperation_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cdb5efb3-5788-509d-9be1-71ccb8a3362a"))
IAsyncOperation<bool> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<bool> __FIAsyncOperation_1_boolean_t;
#define __FIAsyncOperation_1_boolean ABI::Windows::Foundation::__FIAsyncOperation_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_boolean_USE */



#ifndef DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#define DEF___FIAsyncOperationCompletedHandler_1_boolean_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c1d3d1a2-ae17-5a5f-b5a2-bdcc8844889a"))
IAsyncOperationCompletedHandler<bool> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Boolean>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<bool> __FIAsyncOperationCompletedHandler_1_boolean_t;
#define __FIAsyncOperationCompletedHandler_1_boolean ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_boolean_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_boolean_USE */



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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CIMediaMarker_USE
#define DEF___FIIterator_1_Windows__CMedia__CIMediaMarker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f464661e-88bc-5cea-93cd-0c123f17d258"))
IIterator<ABI::Windows::Media::IMediaMarker*> : IIterator_impl<ABI::Windows::Media::IMediaMarker*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.IMediaMarker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::IMediaMarker*> __FIIterator_1_Windows__CMedia__CIMediaMarker_t;
#define __FIIterator_1_Windows__CMedia__CIMediaMarker ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CIMediaMarker_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CIMediaMarker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CIMediaMarker_USE
#define DEF___FIIterable_1_Windows__CMedia__CIMediaMarker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a1c0a397-0364-5e4c-9dca-7cd7011bd114"))
IIterable<ABI::Windows::Media::IMediaMarker*> : IIterable_impl<ABI::Windows::Media::IMediaMarker*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.IMediaMarker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::IMediaMarker*> __FIIterable_1_Windows__CMedia__CIMediaMarker_t;
#define __FIIterable_1_Windows__CMedia__CIMediaMarker ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CIMediaMarker_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CIMediaMarker_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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

#ifndef DEF___FIVectorView_1_Windows__CMedia__CIMediaMarker_USE
#define DEF___FIVectorView_1_Windows__CMedia__CIMediaMarker_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b543562c-02b1-5824-80a8-9854130cdadd"))
IVectorView<ABI::Windows::Media::IMediaMarker*> : IVectorView_impl<ABI::Windows::Media::IMediaMarker*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.IMediaMarker>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::IMediaMarker*> __FIVectorView_1_Windows__CMedia__CIMediaMarker_t;
#define __FIVectorView_1_Windows__CMedia__CIMediaMarker ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CIMediaMarker_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CIMediaMarker_USE */

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

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef struct BitmapBounds BitmapBounds;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_USE
#define DEF___FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("aa19da70-dee6-5b42-b562-2fcd218c34ca"))
IReference<struct ABI::Windows::Graphics::Imaging::BitmapBounds> : IReference_impl<struct ABI::Windows::Graphics::Imaging::BitmapBounds>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Graphics.Imaging.BitmapBounds>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Graphics::Imaging::BitmapBounds> __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_t;
#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds ABI::Windows::Foundation::__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            class MediaTimelineController;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5c43e195-7d39-5d0d-a309-1991e68acdb7"))
ITypedEventHandler<ABI::Windows::Media::MediaTimelineController*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaTimelineController*, ABI::Windows::Media::IMediaTimelineController*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.MediaTimelineController, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::MediaTimelineController*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            class MediaTimelineControllerFailedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("07cd62fb-578d-56a6-a8ef-e653eb005d1b"))
ITypedEventHandler<ABI::Windows::Media::MediaTimelineController*, ABI::Windows::Media::MediaTimelineControllerFailedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaTimelineController*, ABI::Windows::Media::IMediaTimelineController*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaTimelineControllerFailedEventArgs*, ABI::Windows::Media::IMediaTimelineControllerFailedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.MediaTimelineController, Windows.Media.MediaTimelineControllerFailedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::MediaTimelineController*, ABI::Windows::Media::MediaTimelineControllerFailedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            class SystemMediaTransportControls;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class AutoRepeatModeChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a6214bde-02d5-55b3-ab0d-c6031be70da1"))
ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::AutoRepeatModeChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ISystemMediaTransportControls*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::AutoRepeatModeChangeRequestedEventArgs*, ABI::Windows::Media::IAutoRepeatModeChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SystemMediaTransportControls, Windows.Media.AutoRepeatModeChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::AutoRepeatModeChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            class PlaybackPositionChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44e34f15-bdc0-50a7-ace4-39e91fb753f1"))
ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::PlaybackPositionChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ISystemMediaTransportControls*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlaybackPositionChangeRequestedEventArgs*, ABI::Windows::Media::IPlaybackPositionChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SystemMediaTransportControls, Windows.Media.PlaybackPositionChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::PlaybackPositionChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            class PlaybackRateChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("15eb0182-6366-5b9f-bd8c-8ab4fa9d7cd9"))
ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::PlaybackRateChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ISystemMediaTransportControls*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::PlaybackRateChangeRequestedEventArgs*, ABI::Windows::Media::IPlaybackRateChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SystemMediaTransportControls, Windows.Media.PlaybackRateChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::PlaybackRateChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            class ShuffleEnabledChangeRequestedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("17ecea80-27e4-5dae-abb4-c858ad1c5307"))
ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ShuffleEnabledChangeRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ISystemMediaTransportControls*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::ShuffleEnabledChangeRequestedEventArgs*, ABI::Windows::Media::IShuffleEnabledChangeRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SystemMediaTransportControls, Windows.Media.ShuffleEnabledChangeRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ShuffleEnabledChangeRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            class SystemMediaTransportControlsButtonPressedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0557e996-7b23-5bae-aa81-ea0d671143a4"))
ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::SystemMediaTransportControlsButtonPressedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ISystemMediaTransportControls*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControlsButtonPressedEventArgs*, ABI::Windows::Media::ISystemMediaTransportControlsButtonPressedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SystemMediaTransportControls, Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::SystemMediaTransportControlsButtonPressedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            class SystemMediaTransportControlsPropertyChangedEventArgs;
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9fd61dad-1746-5fa1-a908-ef7cb4603c85"))
ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::SystemMediaTransportControlsPropertyChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::ISystemMediaTransportControls*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::SystemMediaTransportControlsPropertyChangedEventArgs*, ABI::Windows::Media::ISystemMediaTransportControlsPropertyChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.SystemMediaTransportControls, Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::SystemMediaTransportControls*, ABI::Windows::Media::SystemMediaTransportControlsPropertyChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                class AppServiceConnection;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace AppService {
                interface IAppServiceConnection;
            } /* AppService */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection ABI::Windows::ApplicationModel::AppService::IAppServiceConnection

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                interface IPropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet ABI::Windows::Foundation::Collections::IPropertySet

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class ValueSet;
            } /* Collections */
        } /* Foundation */
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

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IMemoryBuffer;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIMemoryBuffer ABI::Windows::Foundation::IMemoryBuffer

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

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
            class Uri;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Foundation {
            interface IUriRuntimeClass;
        } /* Foundation */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CFoundation_CIUriRuntimeClass ABI::Windows::Foundation::IUriRuntimeClass

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DDevice;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace DirectX {
                namespace Direct3D11 {
                    interface IDirect3DSurface;
                } /* Direct3D11 */
            } /* DirectX */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

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
            namespace Imaging {
                typedef enum BitmapAlphaMode : int BitmapAlphaMode;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                typedef enum BitmapPixelFormat : int BitmapPixelFormat;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                class SoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Graphics {
            namespace Imaging {
                interface ISoftwareBitmap;
            } /* Imaging */
        } /* Graphics */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap ABI::Windows::Graphics::Imaging::ISoftwareBitmap

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFile ABI::Windows::Storage::IStorageFile

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                class RandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            namespace Streams {
                interface IRandomAccessStreamReference;
            } /* Streams */
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference ABI::Windows::Storage::Streams::IRandomAccessStreamReference

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum AudioBufferAccessMode : int AudioBufferAccessMode;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum MediaPlaybackAutoRepeatMode : int MediaPlaybackAutoRepeatMode;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum MediaPlaybackStatus : int MediaPlaybackStatus;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum MediaPlaybackType : int MediaPlaybackType;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum MediaTimelineControllerState : int MediaTimelineControllerState;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum SoundLevel : int SoundLevel;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum SystemMediaTransportControlsButton : int SystemMediaTransportControlsButton;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum SystemMediaTransportControlsProperty : int SystemMediaTransportControlsProperty;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class AudioBuffer;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class AudioFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class ImageDisplayProperties;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class MusicDisplayProperties;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class SystemMediaTransportControlsDisplayUpdater;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class SystemMediaTransportControlsTimelineProperties;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class VideoDisplayProperties;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class VideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.AudioBufferAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum AudioBufferAccessMode : int
            {
                AudioBufferAccessMode_Read = 0,
                AudioBufferAccessMode_ReadWrite = 1,
                AudioBufferAccessMode_Write = 2,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.AudioProcessing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum AudioProcessing : int
            {
                AudioProcessing_Default = 0,
                AudioProcessing_Raw = 1,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaPlaybackAutoRepeatMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum MediaPlaybackAutoRepeatMode : int
            {
                MediaPlaybackAutoRepeatMode_None = 0,
                MediaPlaybackAutoRepeatMode_Track = 1,
                MediaPlaybackAutoRepeatMode_List = 2,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaPlaybackStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum MediaPlaybackStatus : int
            {
                MediaPlaybackStatus_Closed = 0,
                MediaPlaybackStatus_Changing = 1,
                MediaPlaybackStatus_Stopped = 2,
                MediaPlaybackStatus_Playing = 3,
                MediaPlaybackStatus_Paused = 4,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaPlaybackType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum MediaPlaybackType : int
            {
                MediaPlaybackType_Unknown = 0,
                MediaPlaybackType_Music = 1,
                MediaPlaybackType_Video = 2,
                MediaPlaybackType_Image = 3,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaTimelineControllerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum MediaTimelineControllerState : int
            {
                MediaTimelineControllerState_Paused = 0,
                MediaTimelineControllerState_Running = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                MediaTimelineControllerState_Stalled = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                MediaTimelineControllerState_Error = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.SoundLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum SoundLevel : int
            {
                SoundLevel_Muted = 0,
                SoundLevel_Low = 1,
                SoundLevel_Full = 2,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SystemMediaTransportControlsButton
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum SystemMediaTransportControlsButton : int
            {
                SystemMediaTransportControlsButton_Play = 0,
                SystemMediaTransportControlsButton_Pause = 1,
                SystemMediaTransportControlsButton_Stop = 2,
                SystemMediaTransportControlsButton_Record = 3,
                SystemMediaTransportControlsButton_FastForward = 4,
                SystemMediaTransportControlsButton_Rewind = 5,
                SystemMediaTransportControlsButton_Next = 6,
                SystemMediaTransportControlsButton_Previous = 7,
                SystemMediaTransportControlsButton_ChannelUp = 8,
                SystemMediaTransportControlsButton_ChannelDown = 9,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SystemMediaTransportControlsProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            enum SystemMediaTransportControlsProperty : int
            {
                SystemMediaTransportControlsProperty_SoundLevel = 0,
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaTimeRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            struct MediaTimeRange
            {
                ABI::Windows::Foundation::TimeSpan Start;
                ABI::Windows::Foundation::TimeSpan End;
            };
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IAudioBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AudioBuffer
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IMemoryBuffer
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAudioBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAudioBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAudioBuffer[] = L"Windows.Media.IAudioBuffer";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("35175827-724b-4c6a-b130-f6537f9ae0d0")
            IAudioBuffer : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Capacity(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Length(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Length(
                    UINT32 value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAudioBuffer = __uuidof(IAudioBuffer);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAudioBuffer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAudioBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IAudioFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AudioFrame
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaFrame
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAudioFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAudioFrame[] = L"Windows.Media.IAudioFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("e36ac304-aab2-4277-9ed0-43cedf8e29c6")
            IAudioFrame : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE LockBuffer(
                    ABI::Windows::Media::AudioBufferAccessMode mode,
                    ABI::Windows::Media::IAudioBuffer** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAudioFrame = __uuidof(IAudioFrame);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAudioFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAudioFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IAudioFrameFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AudioFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAudioFrameFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAudioFrameFactory[] = L"Windows.Media.IAudioFrameFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("91a90ade-2422-40a6-b9ad-30d02404317d")
            IAudioFrameFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Create(
                    UINT32 capacity,
                    ABI::Windows::Media::IAudioFrame** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAudioFrameFactory = __uuidof(IAudioFrameFactory);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAudioFrameFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAudioFrameFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IAutoRepeatModeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AutoRepeatModeChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAutoRepeatModeChangeRequestedEventArgs[] = L"Windows.Media.IAutoRepeatModeChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("ea137efa-d852-438e-882b-c990109a78f4")
            IAutoRepeatModeChangeRequestedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_RequestedAutoRepeatMode(
                    ABI::Windows::Media::MediaPlaybackAutoRepeatMode* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAutoRepeatModeChangeRequestedEventArgs = __uuidof(IAutoRepeatModeChangeRequestedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IImageDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ImageDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIImageDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IImageDisplayProperties[] = L"Windows.Media.IImageDisplayProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("cd0bc7ef-54e7-411f-9933-f0e98b0a96d2")
            IImageDisplayProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Title(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Title(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Subtitle(
                    HSTRING value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IImageDisplayProperties = __uuidof(IImageDisplayProperties);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIImageDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIImageDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaControl
 *
 * Introduced to Windows.Media.MediaControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaControl
 *
 */
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaControl[] = L"Windows.Media.IMediaControl";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("98f1fbe1-7a8d-42cb-b6fe-8fe698264f13")
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
            DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
            IMediaControl : public IInspectable
            {
            public:
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_SoundLevelChanged(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_SoundLevelChanged(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_PlayPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_PlayPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_PausePressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_PausePressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_StopPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_StopPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_PlayPauseTogglePressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_PlayPauseTogglePressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_RecordPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_RecordPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_NextTrackPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_NextTrackPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_PreviousTrackPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_PreviousTrackPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_FastForwardPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_FastForwardPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_RewindPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_RewindPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_ChannelUpPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_ChannelUpPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE add_ChannelDownPressed(
                    __FIEventHandler_1_IInspectable* handler,
                    EventRegistrationToken* cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE remove_ChannelDownPressed(
                    EventRegistrationToken cookie
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE get_SoundLevel(
                    ABI::Windows::Media::SoundLevel* value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE put_TrackName(
                    HSTRING value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE get_TrackName(
                    HSTRING* value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE put_ArtistName(
                    HSTRING value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE get_ArtistName(
                    HSTRING* value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE put_IsPlaying(
                    boolean value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE get_IsPlaying(
                    boolean* value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE put_AlbumArt(
                    ABI::Windows::Foundation::IUriRuntimeClass* value
                    ) = 0;
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE get_AlbumArt(
                    ABI::Windows::Foundation::IUriRuntimeClass** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaControl = __uuidof(IMediaControl);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaExtension[] = L"Windows.Media.IMediaExtension";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("07915118-45df-442b-8a3f-f7826a6370ab")
            IMediaExtension : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE SetProperties(
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaExtension = __uuidof(IMediaExtension);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaExtension;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaExtensionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaExtensionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaExtensionManager[] = L"Windows.Media.IMediaExtensionManager";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("4a25eaf5-242d-4dfb-97f4-69b7c42576ff")
            IMediaExtensionManager : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RegisterSchemeHandler(
                    HSTRING activatableClassId,
                    HSTRING scheme
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterSchemeHandlerWithSettings(
                    HSTRING activatableClassId,
                    HSTRING scheme,
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterByteStreamHandler(
                    HSTRING activatableClassId,
                    HSTRING fileExtension,
                    HSTRING mimeType
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterByteStreamHandlerWithSettings(
                    HSTRING activatableClassId,
                    HSTRING fileExtension,
                    HSTRING mimeType,
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterAudioDecoder(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterAudioDecoderWithSettings(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype,
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterAudioEncoder(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterAudioEncoderWithSettings(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype,
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterVideoDecoder(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterVideoDecoderWithSettings(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype,
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterVideoEncoder(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RegisterVideoEncoderWithSettings(
                    HSTRING activatableClassId,
                    GUID inputSubtype,
                    GUID outputSubtype,
                    ABI::Windows::Foundation::Collections::IPropertySet* configuration
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaExtensionManager = __uuidof(IMediaExtensionManager);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaExtensionManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaExtensionManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaExtensionManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtensionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaExtensionManager2[] = L"Windows.Media.IMediaExtensionManager2";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("5bcebf47-4043-4fed-acaf-54ec29dfb1f7")
            IMediaExtensionManager2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RegisterMediaExtensionForAppService(
                    ABI::Windows::Media::IMediaExtension* extension,
                    ABI::Windows::ApplicationModel::AppService::IAppServiceConnection* connection
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaExtensionManager2 = __uuidof(IMediaExtensionManager2);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaExtensionManager2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaFrame[] = L"Windows.Media.IMediaFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("bfb52f8c-5943-47d8-8e10-05308aa5fbd0")
            IMediaFrame : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Type(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_RelativeTime(
                    __FIReference_1_Windows__CFoundation__CTimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RelativeTime(
                    __FIReference_1_Windows__CFoundation__CTimeSpan** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_SystemRelativeTime(
                    __FIReference_1_Windows__CFoundation__CTimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeTime(
                    __FIReference_1_Windows__CFoundation__CTimeSpan** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Duration(
                    __FIReference_1_Windows__CFoundation__CTimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Duration(
                    __FIReference_1_Windows__CFoundation__CTimeSpan** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsDiscontinuous(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsDiscontinuous(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedProperties(
                    ABI::Windows::Foundation::Collections::IPropertySet** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaFrame = __uuidof(IMediaFrame);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaMarker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaMarker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaMarker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaMarker[] = L"Windows.Media.IMediaMarker";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("1803def8-dca5-4b6f-9c20-e3d3c0643625")
            IMediaMarker : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Time(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MediaMarkerType(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Text(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaMarker = __uuidof(IMediaMarker);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaMarker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaMarker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaMarkerTypesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaMarkerTypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaMarkerTypesStatics[] = L"Windows.Media.IMediaMarkerTypesStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("bb198040-482f-4743-8832-45853821ece0")
            IMediaMarkerTypesStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Bookmark(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaMarkerTypesStatics = __uuidof(IMediaMarkerTypesStatics);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaMarkers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaMarkers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaMarkers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaMarkers[] = L"Windows.Media.IMediaMarkers";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("afeab189-f8dd-466e-aa10-920b52353fdf")
            IMediaMarkers : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Markers(
                    __FIVectorView_1_Windows__CMedia__CIMediaMarker** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaMarkers = __uuidof(IMediaMarkers);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaMarkers;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaMarkers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaProcessingTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProcessingTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaProcessingTriggerDetails[] = L"Windows.Media.IMediaProcessingTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("eb8564ac-a351-4f4e-b4f0-9bf2408993db")
            IMediaProcessingTriggerDetails : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Arguments(
                    ABI::Windows::Foundation::Collections::IPropertySet** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaProcessingTriggerDetails = __uuidof(IMediaProcessingTriggerDetails);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaTimelineController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaTimelineController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaTimelineController[] = L"Windows.Media.IMediaTimelineController";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("8ed361f3-0b78-4360-bf71-0c841999ea1b")
            IMediaTimelineController : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE Resume(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Position(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Position(
                    ABI::Windows::Foundation::TimeSpan value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ClockRate(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_ClockRate(
                    DOUBLE value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_State(
                    ABI::Windows::Media::MediaTimelineControllerState* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PositionChanged(
                    __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* positionChangedEventHandler,
                    EventRegistrationToken* eventCookie
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PositionChanged(
                    EventRegistrationToken eventCookie
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                    __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* stateChangedEventHandler,
                    EventRegistrationToken* eventCookie
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                    EventRegistrationToken eventCookie
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaTimelineController = __uuidof(IMediaTimelineController);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaTimelineController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.IMediaTimelineController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaTimelineController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaTimelineController2[] = L"Windows.Media.IMediaTimelineController2";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("ef74ea38-9e72-4df9-8355-6e90c81bbadd")
            IMediaTimelineController2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Duration(
                    __FIReference_1_Windows__CFoundation__CTimeSpan** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Duration(
                    __FIReference_1_Windows__CFoundation__CTimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsLoopingEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsLoopingEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_Failed(
                    __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs* eventHandler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_Failed(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_Ended(
                    __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* eventHandler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_Ended(
                    EventRegistrationToken token
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaTimelineController2 = __uuidof(IMediaTimelineController2);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaTimelineController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IMediaTimelineControllerFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaTimelineControllerFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaTimelineControllerFailedEventArgs[] = L"Windows.Media.IMediaTimelineControllerFailedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("8821f81d-3e77-43fb-be26-4fc87a044834")
            IMediaTimelineControllerFailedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMediaTimelineControllerFailedEventArgs = __uuidof(IMediaTimelineControllerFailedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IMusicDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MusicDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMusicDisplayProperties[] = L"Windows.Media.IMusicDisplayProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("6bbf0c59-d0a0-4d26-92a0-f978e1d18e7b")
            IMusicDisplayProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Title(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Title(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AlbumArtist(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_AlbumArtist(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Artist(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Artist(
                    HSTRING value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMusicDisplayProperties = __uuidof(IMusicDisplayProperties);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMusicDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMusicDisplayProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MusicDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMusicDisplayProperties2[] = L"Windows.Media.IMusicDisplayProperties2";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("00368462-97d3-44b9-b00f-008afcefaf18")
            IMusicDisplayProperties2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AlbumTitle(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_AlbumTitle(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TrackNumber(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_TrackNumber(
                    UINT32 value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Genres(
                    __FIVector_1_HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMusicDisplayProperties2 = __uuidof(IMusicDisplayProperties2);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMusicDisplayProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMusicDisplayProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.MusicDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMusicDisplayProperties3[] = L"Windows.Media.IMusicDisplayProperties3";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("4db51ac1-0681-4e8c-9401-b8159d9eefc7")
            IMusicDisplayProperties3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AlbumTrackCount(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_AlbumTrackCount(
                    UINT32 value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IMusicDisplayProperties3 = __uuidof(IMusicDisplayProperties3);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMusicDisplayProperties3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.IPlaybackPositionChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlaybackPositionChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IPlaybackPositionChangeRequestedEventArgs[] = L"Windows.Media.IPlaybackPositionChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("b4493f88-eb28-4961-9c14-335e44f3e125")
            IPlaybackPositionChangeRequestedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_RequestedPlaybackPosition(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPlaybackPositionChangeRequestedEventArgs = __uuidof(IPlaybackPositionChangeRequestedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IPlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlaybackRateChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IPlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.IPlaybackRateChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("2ce2c41f-3cd6-4f77-9ba7-eb27c26a2140")
            IPlaybackRateChangeRequestedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_RequestedPlaybackRate(
                    DOUBLE* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPlaybackRateChangeRequestedEventArgs = __uuidof(IPlaybackRateChangeRequestedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IShuffleEnabledChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ShuffleEnabledChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IShuffleEnabledChangeRequestedEventArgs[] = L"Windows.Media.IShuffleEnabledChangeRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("49b593fe-4fd0-4666-a314-c0e01940d302")
            IShuffleEnabledChangeRequestedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_RequestedShuffleEnabled(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IShuffleEnabledChangeRequestedEventArgs = __uuidof(IShuffleEnabledChangeRequestedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControls
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControls
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControls[] = L"Windows.Media.ISystemMediaTransportControls";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("99fa3ff4-1742-42a6-902e-087d41f965ec")
            ISystemMediaTransportControls : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_PlaybackStatus(
                    ABI::Windows::Media::MediaPlaybackStatus* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_PlaybackStatus(
                    ABI::Windows::Media::MediaPlaybackStatus value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayUpdater(
                    ABI::Windows::Media::ISystemMediaTransportControlsDisplayUpdater** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SoundLevel(
                    ABI::Windows::Media::SoundLevel* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsPlayEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsPlayEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsStopEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsStopEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsPauseEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsPauseEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsRecordEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsRecordEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsFastForwardEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsFastForwardEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsRewindEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsRewindEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsPreviousEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsPreviousEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsNextEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsNextEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsChannelUpEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsChannelUpEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsChannelDownEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IsChannelDownEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_ButtonPressed(
                    __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_ButtonPressed(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PropertyChanged(
                    __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PropertyChanged(
                    EventRegistrationToken token
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControls = __uuidof(ISystemMediaTransportControls);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControls;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControls2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControls
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControls2[] = L"Windows.Media.ISystemMediaTransportControls2";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("ea98d2f6-7f3c-4af2-a586-72889808efb1")
            ISystemMediaTransportControls2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_AutoRepeatMode(
                    ABI::Windows::Media::MediaPlaybackAutoRepeatMode* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_AutoRepeatMode(
                    ABI::Windows::Media::MediaPlaybackAutoRepeatMode value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ShuffleEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_ShuffleEnabled(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PlaybackRate(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_PlaybackRate(
                    DOUBLE value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE UpdateTimelineProperties(
                    ABI::Windows::Media::ISystemMediaTransportControlsTimelineProperties* timelineProperties
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PlaybackPositionChangeRequested(
                    __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PlaybackPositionChangeRequested(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PlaybackRateChangeRequested(
                    __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PlaybackRateChangeRequested(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_ShuffleEnabledChangeRequested(
                    __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_ShuffleEnabledChangeRequested(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_AutoRepeatModeChangeRequested(
                    __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_AutoRepeatModeChangeRequested(
                    EventRegistrationToken token
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControls2 = __uuidof(ISystemMediaTransportControls2);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControls2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsButtonPressedEventArgs[] = L"Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("b7f47116-a56f-4dc8-9e11-92031f4a87c2")
            ISystemMediaTransportControlsButtonPressedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Button(
                    ABI::Windows::Media::SystemMediaTransportControlsButton* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControlsButtonPressedEventArgs = __uuidof(ISystemMediaTransportControlsButtonPressedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsDisplayUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsDisplayUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsDisplayUpdater[] = L"Windows.Media.ISystemMediaTransportControlsDisplayUpdater";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("8abbc53e-fa55-4ecf-ad8e-c984e5dd1550")
            ISystemMediaTransportControlsDisplayUpdater : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Type(
                    ABI::Windows::Media::MediaPlaybackType* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Type(
                    ABI::Windows::Media::MediaPlaybackType value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AppMediaId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_AppMediaId(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Thumbnail(
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Thumbnail(
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MusicProperties(
                    ABI::Windows::Media::IMusicDisplayProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_VideoProperties(
                    ABI::Windows::Media::IVideoDisplayProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ImageProperties(
                    ABI::Windows::Media::IImageDisplayProperties** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyFromFileAsync(
                    ABI::Windows::Media::MediaPlaybackType type,
                    ABI::Windows::Storage::IStorageFile* source,
                    __FIAsyncOperation_1_boolean** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE ClearAll(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE Update(void) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControlsDisplayUpdater = __uuidof(ISystemMediaTransportControlsDisplayUpdater);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsPropertyChangedEventArgs[] = L"Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("d0ca0936-339b-4cb3-8eeb-737607f56e08")
            ISystemMediaTransportControlsPropertyChangedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Property(
                    ABI::Windows::Media::SystemMediaTransportControlsProperty* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControlsPropertyChangedEventArgs = __uuidof(ISystemMediaTransportControlsPropertyChangedEventArgs);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControls
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsStatics[] = L"Windows.Media.ISystemMediaTransportControlsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("43ba380a-eca4-4832-91ab-d415fae484c6")
            ISystemMediaTransportControlsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetForCurrentView(
                    ABI::Windows::Media::ISystemMediaTransportControls** mediaControl
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControlsStatics = __uuidof(ISystemMediaTransportControlsStatics);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsTimelineProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsTimelineProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsTimelineProperties[] = L"Windows.Media.ISystemMediaTransportControlsTimelineProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("5125316a-c3a2-475b-8507-93534dc88f15")
            ISystemMediaTransportControlsTimelineProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                    ABI::Windows::Foundation::TimeSpan value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EndTime(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_EndTime(
                    ABI::Windows::Foundation::TimeSpan value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MinSeekTime(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_MinSeekTime(
                    ABI::Windows::Foundation::TimeSpan value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MaxSeekTime(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_MaxSeekTime(
                    ABI::Windows::Foundation::TimeSpan value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Position(
                    ABI::Windows::Foundation::TimeSpan* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Position(
                    ABI::Windows::Foundation::TimeSpan value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISystemMediaTransportControlsTimelineProperties = __uuidof(ISystemMediaTransportControlsTimelineProperties);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoDisplayProperties[] = L"Windows.Media.IVideoDisplayProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("5609fdb1-5d2d-4872-8170-45dee5bc2f5c")
            IVideoDisplayProperties : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Title(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Title(
                    HSTRING value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Subtitle(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Subtitle(
                    HSTRING value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoDisplayProperties = __uuidof(IVideoDisplayProperties);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoDisplayProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoDisplayProperties2[] = L"Windows.Media.IVideoDisplayProperties2";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("b410e1ce-ab52-41ab-a486-cc10fab152f9")
            IVideoDisplayProperties2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Genres(
                    __FIVector_1_HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoDisplayProperties2 = __uuidof(IVideoDisplayProperties2);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoDisplayProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoEffectsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoEffects
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoEffectsStatics[] = L"Windows.Media.IVideoEffectsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("1fcda5e8-baf1-4521-980c-3bcebb44cf38")
            IVideoEffectsStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_VideoStabilization(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoEffectsStatics = __uuidof(IVideoEffectsStatics);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoEffectsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaFrame
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrame[] = L"Windows.Media.IVideoFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("0cc06625-90fc-4c92-bd95-7ded21819d1c")
            IVideoFrame : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_SoftwareBitmap(
                    ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CopyToAsync(
                    ABI::Windows::Media::IVideoFrame* frame,
                    ABI::Windows::Foundation::IAsyncAction** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Direct3DSurface(
                    ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoFrame = __uuidof(IVideoFrame);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrame2[] = L"Windows.Media.IVideoFrame2";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("3837840d-336c-4366-8d46-060798736c5d")
            IVideoFrame2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CopyToWithBoundsAsync(
                    ABI::Windows::Media::IVideoFrame* frame,
                    __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* sourceBounds,
                    __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* destinationBounds,
                    ABI::Windows::Foundation::IAsyncAction** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoFrame2 = __uuidof(IVideoFrame2);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrame2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.IVideoFrameFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrameFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrameFactory[] = L"Windows.Media.IVideoFrameFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("014b6d69-2228-4c92-92ff-50c380d3e776")
            IVideoFrameFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Create(
                    ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                    INT32 width,
                    INT32 height,
                    ABI::Windows::Media::IVideoFrame** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateWithAlpha(
                    ABI::Windows::Graphics::Imaging::BitmapPixelFormat format,
                    INT32 width,
                    INT32 height,
                    ABI::Windows::Graphics::Imaging::BitmapAlphaMode alpha,
                    ABI::Windows::Media::IVideoFrame** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoFrameFactory = __uuidof(IVideoFrameFactory);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrameFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrameFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoFrameStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrameStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrameStatics[] = L"Windows.Media.IVideoFrameStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            MIDL_INTERFACE("ab2a556f-6111-4b33-8ec3-2b209a02e17a")
            IVideoFrameStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateAsDirect3D11SurfaceBacked(
                    ABI::Windows::Graphics::DirectX::DirectXPixelFormat format,
                    INT32 width,
                    INT32 height,
                    ABI::Windows::Media::IVideoFrame** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateAsDirect3D11SurfaceBackedWithDevice(
                    ABI::Windows::Graphics::DirectX::DirectXPixelFormat format,
                    INT32 width,
                    INT32 height,
                    ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice* device,
                    ABI::Windows::Media::IVideoFrame** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateWithSoftwareBitmap(
                    ABI::Windows::Graphics::Imaging::ISoftwareBitmap* bitmap,
                    ABI::Windows::Media::IVideoFrame** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CreateWithDirect3D11Surface(
                    ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface* surface,
                    ABI::Windows::Media::IVideoFrame** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IVideoFrameStatics = __uuidof(IVideoFrameStatics);
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrameStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrameStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.AudioBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IAudioBuffer ** Default Interface **
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AudioBuffer_DEFINED
#define RUNTIMECLASS_Windows_Media_AudioBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AudioBuffer[] = L"Windows.Media.AudioBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AudioFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.IAudioFrameFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IAudioFrame ** Default Interface **
 *    Windows.Media.IMediaFrame
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AudioFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_AudioFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AudioFrame[] = L"Windows.Media.AudioFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AutoRepeatModeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IAutoRepeatModeChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AutoRepeatModeChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_AutoRepeatModeChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AutoRepeatModeChangeRequestedEventArgs[] = L"Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ImageDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IImageDisplayProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ImageDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_ImageDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ImageDisplayProperties[] = L"Windows.Media.ImageDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaControl
 *
 * Introduced to Windows.Media.MediaControlContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IMediaControl interface starting with version 1.0 of the Windows.Media.MediaControlContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaControl_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaControl_DEFINED
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaControl[] = L"Windows.Media.MediaControl";
#endif
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaExtensionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaExtensionManager ** Default Interface **
 *    Windows.Media.IMediaExtensionManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaExtensionManager_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaExtensionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaExtensionManager[] = L"Windows.Media.MediaExtensionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaMarkerTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IMediaMarkerTypesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaMarkerTypes_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaMarkerTypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaMarkerTypes[] = L"Windows.Media.MediaMarkerTypes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProcessingTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaProcessingTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProcessingTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProcessingTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProcessingTriggerDetails[] = L"Windows.Media.MediaProcessingTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaTimelineController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaTimelineController ** Default Interface **
 *    Windows.Media.IMediaTimelineController2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_MediaTimelineController_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaTimelineController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaTimelineController[] = L"Windows.Media.MediaTimelineController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.MediaTimelineControllerFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaTimelineControllerFailedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_MediaTimelineControllerFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaTimelineControllerFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaTimelineControllerFailedEventArgs[] = L"Windows.Media.MediaTimelineControllerFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.MusicDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMusicDisplayProperties ** Default Interface **
 *    Windows.Media.IMusicDisplayProperties2
 *    Windows.Media.IMusicDisplayProperties3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MusicDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MusicDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MusicDisplayProperties[] = L"Windows.Media.MusicDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlaybackPositionChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IPlaybackPositionChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlaybackPositionChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlaybackPositionChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlaybackPositionChangeRequestedEventArgs[] = L"Windows.Media.PlaybackPositionChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IPlaybackRateChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlaybackRateChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlaybackRateChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.PlaybackRateChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ShuffleEnabledChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IShuffleEnabledChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ShuffleEnabledChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_ShuffleEnabledChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ShuffleEnabledChangeRequestedEventArgs[] = L"Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControls
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.ISystemMediaTransportControlsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControls ** Default Interface **
 *    Windows.Media.ISystemMediaTransportControls2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControls_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControls_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControls[] = L"Windows.Media.SystemMediaTransportControls";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsButtonPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsButtonPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsButtonPressedEventArgs[] = L"Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsDisplayUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsDisplayUpdater ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsDisplayUpdater_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsDisplayUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsDisplayUpdater[] = L"Windows.Media.SystemMediaTransportControlsDisplayUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsPropertyChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsPropertyChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsPropertyChangedEventArgs[] = L"Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsTimelineProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsTimelineProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsTimelineProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsTimelineProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsTimelineProperties[] = L"Windows.Media.SystemMediaTransportControlsTimelineProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.VideoDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IVideoDisplayProperties ** Default Interface **
 *    Windows.Media.IVideoDisplayProperties2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_VideoDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_VideoDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_VideoDisplayProperties[] = L"Windows.Media.VideoDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.VideoEffects
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IVideoEffectsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_VideoEffects_DEFINED
#define RUNTIMECLASS_Windows_Media_VideoEffects_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_VideoEffects[] = L"Windows.Media.VideoEffects";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.VideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.IVideoFrameFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IVideoFrameStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IVideoFrame ** Default Interface **
 *    Windows.Media.IMediaFrame
 *    Windows.Foundation.IClosable
 *    Windows.Media.IVideoFrame2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_VideoFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_VideoFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_VideoFrame[] = L"Windows.Media.VideoFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CIAudioBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAudioBuffer __x_ABI_CWindows_CMedia_CIAudioBuffer;

#endif // ____x_ABI_CWindows_CMedia_CIAudioBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAudioFrame __x_ABI_CWindows_CMedia_CIAudioFrame;

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAudioFrameFactory __x_ABI_CWindows_CMedia_CIAudioFrameFactory;

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIImageDisplayProperties __x_ABI_CWindows_CMedia_CIImageDisplayProperties;

#endif // ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaControl_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaControl_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaControl __x_ABI_CWindows_CMedia_CIMediaControl;

#endif // ____x_ABI_CWindows_CMedia_CIMediaControl_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaExtension __x_ABI_CWindows_CMedia_CIMediaExtension;

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtension_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaExtensionManager __x_ABI_CWindows_CMedia_CIMediaExtensionManager;

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaExtensionManager2 __x_ABI_CWindows_CMedia_CIMediaExtensionManager2;

#endif // ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaFrame __x_ABI_CWindows_CMedia_CIMediaFrame;

#endif // ____x_ABI_CWindows_CMedia_CIMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaMarker __x_ABI_CWindows_CMedia_CIMediaMarker;

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarker_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics;

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaMarkers_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaMarkers_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaMarkers __x_ABI_CWindows_CMedia_CIMediaMarkers;

#endif // ____x_ABI_CWindows_CMedia_CIMediaMarkers_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails;

#endif // ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaTimelineController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaTimelineController __x_ABI_CWindows_CMedia_CIMediaTimelineController;

#endif // ____x_ABI_CWindows_CMedia_CIMediaTimelineController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaTimelineController2 __x_ABI_CWindows_CMedia_CIMediaTimelineController2;

#endif // ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMusicDisplayProperties __x_ABI_CWindows_CMedia_CIMusicDisplayProperties;

#endif // ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2 __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2;

#endif // ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3 __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3;

#endif // ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControls __x_ABI_CWindows_CMedia_CISystemMediaTransportControls;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2 __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties;

#endif // ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoDisplayProperties __x_ABI_CWindows_CMedia_CIVideoDisplayProperties;

#endif // ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2 __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2;

#endif // ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoEffectsStatics __x_ABI_CWindows_CMedia_CIVideoEffectsStatics;

#endif // ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame __x_ABI_CWindows_CMedia_CIVideoFrame;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame2 __x_ABI_CWindows_CMedia_CIVideoFrame2;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrameFactory __x_ABI_CWindows_CMedia_CIVideoFrameFactory;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrameStatics __x_ABI_CWindows_CMedia_CIVideoFrameStatics;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

#if !defined(____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_boolean __FIAsyncOperation_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_boolean;

typedef struct __FIAsyncOperation_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_boolean* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_boolean* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_boolean* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_boolean* This,
        __FIAsyncOperationCompletedHandler_1_boolean** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_boolean* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperation_1_booleanVtbl;

interface __FIAsyncOperation_1_boolean
{
    CONST_VTBL struct __FIAsyncOperation_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_boolean_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_boolean_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_boolean_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_boolean_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_boolean_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_boolean_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_boolean_INTERFACE_DEFINED__

#if !defined(____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_boolean __FIAsyncOperationCompletedHandler_1_boolean;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_boolean;

typedef struct __FIAsyncOperationCompletedHandler_1_booleanVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_boolean* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_boolean* This,
        __FIAsyncOperation_1_boolean* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_booleanVtbl;

interface __FIAsyncOperationCompletedHandler_1_boolean
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_booleanVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_boolean_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_boolean_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_boolean_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_boolean_INTERFACE_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CIMediaMarker __FIIterator_1_Windows__CMedia__CIMediaMarker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CIMediaMarker;

typedef struct __FIIterator_1_Windows__CMedia__CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        __x_ABI_CWindows_CMedia_CIMediaMarker** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CIMediaMarker* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CIMediaMarker** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CIMediaMarkerVtbl;

interface __FIIterator_1_Windows__CMedia__CIMediaMarker
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CIMediaMarker_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CIMediaMarker __FIIterable_1_Windows__CMedia__CIMediaMarker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CIMediaMarker;

typedef struct __FIIterable_1_Windows__CMedia__CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CIMediaMarker* This,
        __FIIterator_1_Windows__CMedia__CIMediaMarker** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CIMediaMarkerVtbl;

interface __FIIterable_1_Windows__CMedia__CIMediaMarker
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CIMediaMarker_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
#if !defined(____FIVectorView_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CIMediaMarker __FIVectorView_1_Windows__CMedia__CIMediaMarker;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CIMediaMarker;

typedef struct __FIVectorView_1_Windows__CMedia__CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CIMediaMarker** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        __x_ABI_CWindows_CMedia_CIMediaMarker* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CIMediaMarker* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CIMediaMarker** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CIMediaMarkerVtbl;

interface __FIVectorView_1_Windows__CMedia__CIMediaMarker
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CIMediaMarker_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CIMediaMarker_INTERFACE_DEFINED__
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

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds;

typedef struct __FIReference_1_Windows__CGraphics__CImaging__CBitmapBoundsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* This,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapBounds* result);

    END_INTERFACE
} __FIReference_1_Windows__CGraphics__CImaging__CBitmapBoundsVtbl;

interface __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds
{
    CONST_VTBL struct __FIReference_1_Windows__CGraphics__CImaging__CBitmapBoundsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* This,
        __x_ABI_CWindows_CMedia_CIMediaTimelineController* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs* This,
        __x_ABI_CWindows_CMedia_CIMediaTimelineController* sender,
        __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls* sender,
        __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls* sender,
        __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls* sender,
        __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls* sender,
        __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls* sender,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls* sender,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection;

#endif // ____x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIMemoryBuffer __x_ABI_CWindows_CFoundation_CIMemoryBuffer;

#endif // ____x_ABI_CWindows_CFoundation_CIMemoryBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode;

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CAudioBufferAccessMode __x_ABI_CWindows_CMedia_CAudioBufferAccessMode;

typedef enum __x_ABI_CWindows_CMedia_CMediaPlaybackAutoRepeatMode __x_ABI_CWindows_CMedia_CMediaPlaybackAutoRepeatMode;

typedef enum __x_ABI_CWindows_CMedia_CMediaPlaybackStatus __x_ABI_CWindows_CMedia_CMediaPlaybackStatus;

typedef enum __x_ABI_CWindows_CMedia_CMediaPlaybackType __x_ABI_CWindows_CMedia_CMediaPlaybackType;

typedef enum __x_ABI_CWindows_CMedia_CMediaTimelineControllerState __x_ABI_CWindows_CMedia_CMediaTimelineControllerState;

typedef enum __x_ABI_CWindows_CMedia_CSoundLevel __x_ABI_CWindows_CMedia_CSoundLevel;

typedef enum __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsButton __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsButton;

typedef enum __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsProperty __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsProperty;

/*
 *
 * Struct Windows.Media.AudioBufferAccessMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudioBufferAccessMode
{
    AudioBufferAccessMode_Read = 0,
    AudioBufferAccessMode_ReadWrite = 1,
    AudioBufferAccessMode_Write = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.AudioProcessing
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudioProcessing
{
    AudioProcessing_Default = 0,
    AudioProcessing_Raw = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaPlaybackAutoRepeatMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaPlaybackAutoRepeatMode
{
    MediaPlaybackAutoRepeatMode_None = 0,
    MediaPlaybackAutoRepeatMode_Track = 1,
    MediaPlaybackAutoRepeatMode_List = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaPlaybackStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaPlaybackStatus
{
    MediaPlaybackStatus_Closed = 0,
    MediaPlaybackStatus_Changing = 1,
    MediaPlaybackStatus_Stopped = 2,
    MediaPlaybackStatus_Playing = 3,
    MediaPlaybackStatus_Paused = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaPlaybackType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaPlaybackType
{
    MediaPlaybackType_Unknown = 0,
    MediaPlaybackType_Music = 1,
    MediaPlaybackType_Video = 2,
    MediaPlaybackType_Image = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaTimelineControllerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CMediaTimelineControllerState
{
    MediaTimelineControllerState_Paused = 0,
    MediaTimelineControllerState_Running = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    MediaTimelineControllerState_Stalled = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    MediaTimelineControllerState_Error = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.SoundLevel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSoundLevel
{
    SoundLevel_Muted = 0,
    SoundLevel_Low = 1,
    SoundLevel_Full = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SystemMediaTransportControlsButton
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsButton
{
    SystemMediaTransportControlsButton_Play = 0,
    SystemMediaTransportControlsButton_Pause = 1,
    SystemMediaTransportControlsButton_Stop = 2,
    SystemMediaTransportControlsButton_Record = 3,
    SystemMediaTransportControlsButton_FastForward = 4,
    SystemMediaTransportControlsButton_Rewind = 5,
    SystemMediaTransportControlsButton_Next = 6,
    SystemMediaTransportControlsButton_Previous = 7,
    SystemMediaTransportControlsButton_ChannelUp = 8,
    SystemMediaTransportControlsButton_ChannelDown = 9,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.SystemMediaTransportControlsProperty
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsProperty
{
    SystemMediaTransportControlsProperty_SoundLevel = 0,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaTimeRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
struct __x_ABI_CWindows_CMedia_CMediaTimeRange
{
    struct __x_ABI_CWindows_CFoundation_CTimeSpan Start;
    struct __x_ABI_CWindows_CFoundation_CTimeSpan End;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IAudioBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AudioBuffer
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IMemoryBuffer
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAudioBuffer_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAudioBuffer_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAudioBuffer[] = L"Windows.Media.IAudioBuffer";
typedef struct __x_ABI_CWindows_CMedia_CIAudioBufferVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Capacity)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Length)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Length)(__x_ABI_CWindows_CMedia_CIAudioBuffer* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIAudioBufferVtbl;

interface __x_ABI_CWindows_CMedia_CIAudioBuffer
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIAudioBufferVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_get_Capacity(This, value) \
    ((This)->lpVtbl->get_Capacity(This, value))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_get_Length(This, value) \
    ((This)->lpVtbl->get_Length(This, value))

#define __x_ABI_CWindows_CMedia_CIAudioBuffer_put_Length(This, value) \
    ((This)->lpVtbl->put_Length(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAudioBuffer;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAudioBuffer_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IAudioFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AudioFrame
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaFrame
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAudioFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAudioFrame[] = L"Windows.Media.IAudioFrame";
typedef struct __x_ABI_CWindows_CMedia_CIAudioFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIAudioFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIAudioFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIAudioFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIAudioFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIAudioFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIAudioFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LockBuffer)(__x_ABI_CWindows_CMedia_CIAudioFrame* This,
        enum __x_ABI_CWindows_CMedia_CAudioBufferAccessMode mode,
        __x_ABI_CWindows_CMedia_CIAudioBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIAudioFrameVtbl;

interface __x_ABI_CWindows_CMedia_CIAudioFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIAudioFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIAudioFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIAudioFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIAudioFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIAudioFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIAudioFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIAudioFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIAudioFrame_LockBuffer(This, mode, value) \
    ((This)->lpVtbl->LockBuffer(This, mode, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAudioFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAudioFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IAudioFrameFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AudioFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAudioFrameFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAudioFrameFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAudioFrameFactory[] = L"Windows.Media.IAudioFrameFactory";
typedef struct __x_ABI_CWindows_CMedia_CIAudioFrameFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CIAudioFrameFactory* This,
        UINT32 capacity,
        __x_ABI_CWindows_CMedia_CIAudioFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIAudioFrameFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CIAudioFrameFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIAudioFrameFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIAudioFrameFactory_Create(This, capacity, value) \
    ((This)->lpVtbl->Create(This, capacity, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAudioFrameFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAudioFrameFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IAutoRepeatModeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.AutoRepeatModeChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IAutoRepeatModeChangeRequestedEventArgs[] = L"Windows.Media.IAutoRepeatModeChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestedAutoRepeatMode)(__x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackAutoRepeatMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_get_RequestedAutoRepeatMode(This, value) \
    ((This)->lpVtbl->get_RequestedAutoRepeatMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIAutoRepeatModeChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IImageDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ImageDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIImageDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIImageDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IImageDisplayProperties[] = L"Windows.Media.IImageDisplayProperties";
typedef struct __x_ABI_CWindows_CMedia_CIImageDisplayPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subtitle)(__x_ABI_CWindows_CMedia_CIImageDisplayProperties* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIImageDisplayPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CIImageDisplayProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIImageDisplayPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CMedia_CIImageDisplayProperties_put_Subtitle(This, value) \
    ((This)->lpVtbl->put_Subtitle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIImageDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIImageDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaControl
 *
 * Introduced to Windows.Media.MediaControlContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaControl
 *
 */
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaControl_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaControl_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaControl[] = L"Windows.Media.IMediaControl";
typedef struct
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
__x_ABI_CWindows_CMedia_CIMediaControlVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaControl* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaControl* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        TrustLevel* trustLevel);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_SoundLevelChanged)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_SoundLevelChanged)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_PlayPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_PlayPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_PausePressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_PausePressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_StopPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_StopPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_PlayPauseTogglePressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_PlayPauseTogglePressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_RecordPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_RecordPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_NextTrackPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_NextTrackPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_PreviousTrackPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_PreviousTrackPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_FastForwardPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_FastForwardPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_RewindPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_RewindPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_ChannelUpPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_ChannelUpPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* add_ChannelDownPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __FIEventHandler_1_IInspectable* handler,
        EventRegistrationToken* cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* remove_ChannelDownPressed)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        EventRegistrationToken cookie);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_SoundLevel)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        enum __x_ABI_CWindows_CMedia_CSoundLevel* value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_TrackName)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        HSTRING value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_TrackName)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        HSTRING* value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_ArtistName)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        HSTRING value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_ArtistName)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        HSTRING* value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_IsPlaying)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        boolean value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_IsPlaying)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        boolean* value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_AlbumArt)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_AlbumArt)(__x_ABI_CWindows_CMedia_CIMediaControl* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaControlVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaControl
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaControlVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaControl_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaControl_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaControl_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaControl_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaControl_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaControl_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_SoundLevelChanged(This, handler, cookie) \
    ((This)->lpVtbl->add_SoundLevelChanged(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_SoundLevelChanged(This, cookie) \
    ((This)->lpVtbl->remove_SoundLevelChanged(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_PlayPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PlayPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_PlayPressed(This, cookie) \
    ((This)->lpVtbl->remove_PlayPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_PausePressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PausePressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_PausePressed(This, cookie) \
    ((This)->lpVtbl->remove_PausePressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_StopPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_StopPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_StopPressed(This, cookie) \
    ((This)->lpVtbl->remove_StopPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_PlayPauseTogglePressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PlayPauseTogglePressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_PlayPauseTogglePressed(This, cookie) \
    ((This)->lpVtbl->remove_PlayPauseTogglePressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_RecordPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_RecordPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_RecordPressed(This, cookie) \
    ((This)->lpVtbl->remove_RecordPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_NextTrackPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_NextTrackPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_NextTrackPressed(This, cookie) \
    ((This)->lpVtbl->remove_NextTrackPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_PreviousTrackPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_PreviousTrackPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_PreviousTrackPressed(This, cookie) \
    ((This)->lpVtbl->remove_PreviousTrackPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_FastForwardPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_FastForwardPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_FastForwardPressed(This, cookie) \
    ((This)->lpVtbl->remove_FastForwardPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_RewindPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_RewindPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_RewindPressed(This, cookie) \
    ((This)->lpVtbl->remove_RewindPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_ChannelUpPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_ChannelUpPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_ChannelUpPressed(This, cookie) \
    ((This)->lpVtbl->remove_ChannelUpPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_add_ChannelDownPressed(This, handler, cookie) \
    ((This)->lpVtbl->add_ChannelDownPressed(This, handler, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_remove_ChannelDownPressed(This, cookie) \
    ((This)->lpVtbl->remove_ChannelDownPressed(This, cookie))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_get_SoundLevel(This, value) \
    ((This)->lpVtbl->get_SoundLevel(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_put_TrackName(This, value) \
    ((This)->lpVtbl->put_TrackName(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_get_TrackName(This, value) \
    ((This)->lpVtbl->get_TrackName(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_put_ArtistName(This, value) \
    ((This)->lpVtbl->put_ArtistName(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_get_ArtistName(This, value) \
    ((This)->lpVtbl->get_ArtistName(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_put_IsPlaying(This, value) \
    ((This)->lpVtbl->put_IsPlaying(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_get_IsPlaying(This, value) \
    ((This)->lpVtbl->get_IsPlaying(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_put_AlbumArt(This, value) \
    ((This)->lpVtbl->put_AlbumArt(This, value))

#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
    DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CMedia_CIMediaControl_get_AlbumArt(This, value) \
    ((This)->lpVtbl->get_AlbumArt(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaControl;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaControl_INTERFACE_DEFINED__) */
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaExtension
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaExtension_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaExtension_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaExtension[] = L"Windows.Media.IMediaExtension";
typedef struct __x_ABI_CWindows_CMedia_CIMediaExtensionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaExtension* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaExtension* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaExtension* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaExtension* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaExtension* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaExtension* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetProperties)(__x_ABI_CWindows_CMedia_CIMediaExtension* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaExtensionVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaExtension
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaExtensionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaExtension_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaExtension_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaExtension_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaExtension_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaExtension_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaExtension_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaExtension_SetProperties(This, configuration) \
    ((This)->lpVtbl->SetProperties(This, configuration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaExtension;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaExtension_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaExtensionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaExtensionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaExtensionManager[] = L"Windows.Media.IMediaExtensionManager";
typedef struct __x_ABI_CWindows_CMedia_CIMediaExtensionManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterSchemeHandler)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        HSTRING scheme);
    HRESULT (STDMETHODCALLTYPE* RegisterSchemeHandlerWithSettings)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        HSTRING scheme,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* RegisterByteStreamHandler)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        HSTRING fileExtension,
        HSTRING mimeType);
    HRESULT (STDMETHODCALLTYPE* RegisterByteStreamHandlerWithSettings)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        HSTRING fileExtension,
        HSTRING mimeType,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* RegisterAudioDecoder)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype);
    HRESULT (STDMETHODCALLTYPE* RegisterAudioDecoderWithSettings)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* RegisterAudioEncoder)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype);
    HRESULT (STDMETHODCALLTYPE* RegisterAudioEncoderWithSettings)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* RegisterVideoDecoder)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype);
    HRESULT (STDMETHODCALLTYPE* RegisterVideoDecoderWithSettings)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);
    HRESULT (STDMETHODCALLTYPE* RegisterVideoEncoder)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype);
    HRESULT (STDMETHODCALLTYPE* RegisterVideoEncoderWithSettings)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager* This,
        HSTRING activatableClassId,
        GUID inputSubtype,
        GUID outputSubtype,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaExtensionManagerVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaExtensionManager
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaExtensionManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterSchemeHandler(This, activatableClassId, scheme) \
    ((This)->lpVtbl->RegisterSchemeHandler(This, activatableClassId, scheme))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterSchemeHandlerWithSettings(This, activatableClassId, scheme, configuration) \
    ((This)->lpVtbl->RegisterSchemeHandlerWithSettings(This, activatableClassId, scheme, configuration))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterByteStreamHandler(This, activatableClassId, fileExtension, mimeType) \
    ((This)->lpVtbl->RegisterByteStreamHandler(This, activatableClassId, fileExtension, mimeType))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterByteStreamHandlerWithSettings(This, activatableClassId, fileExtension, mimeType, configuration) \
    ((This)->lpVtbl->RegisterByteStreamHandlerWithSettings(This, activatableClassId, fileExtension, mimeType, configuration))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterAudioDecoder(This, activatableClassId, inputSubtype, outputSubtype) \
    ((This)->lpVtbl->RegisterAudioDecoder(This, activatableClassId, inputSubtype, outputSubtype))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterAudioDecoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration) \
    ((This)->lpVtbl->RegisterAudioDecoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterAudioEncoder(This, activatableClassId, inputSubtype, outputSubtype) \
    ((This)->lpVtbl->RegisterAudioEncoder(This, activatableClassId, inputSubtype, outputSubtype))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterAudioEncoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration) \
    ((This)->lpVtbl->RegisterAudioEncoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterVideoDecoder(This, activatableClassId, inputSubtype, outputSubtype) \
    ((This)->lpVtbl->RegisterVideoDecoder(This, activatableClassId, inputSubtype, outputSubtype))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterVideoDecoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration) \
    ((This)->lpVtbl->RegisterVideoDecoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterVideoEncoder(This, activatableClassId, inputSubtype, outputSubtype) \
    ((This)->lpVtbl->RegisterVideoEncoder(This, activatableClassId, inputSubtype, outputSubtype))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager_RegisterVideoEncoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration) \
    ((This)->lpVtbl->RegisterVideoEncoderWithSettings(This, activatableClassId, inputSubtype, outputSubtype, configuration))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaExtensionManager;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaExtensionManager2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaExtensionManager
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaExtensionManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaExtensionManager2[] = L"Windows.Media.IMediaExtensionManager2";
typedef struct __x_ABI_CWindows_CMedia_CIMediaExtensionManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RegisterMediaExtensionForAppService)(__x_ABI_CWindows_CMedia_CIMediaExtensionManager2* This,
        __x_ABI_CWindows_CMedia_CIMediaExtension* extension,
        __x_ABI_CWindows_CApplicationModel_CAppService_CIAppServiceConnection* connection);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaExtensionManager2Vtbl;

interface __x_ABI_CWindows_CMedia_CIMediaExtensionManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaExtensionManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaExtensionManager2_RegisterMediaExtensionForAppService(This, extension, connection) \
    ((This)->lpVtbl->RegisterMediaExtensionForAppService(This, extension, connection))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaExtensionManager2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaExtensionManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaFrame[] = L"Windows.Media.IMediaFrame";
typedef struct __x_ABI_CWindows_CMedia_CIMediaFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_RelativeTime)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_RelativeTime)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_SystemRelativeTime)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeTime)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_IsDiscontinuous)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsDiscontinuous)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedProperties)(__x_ABI_CWindows_CMedia_CIMediaFrame* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaFrameVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_put_RelativeTime(This, value) \
    ((This)->lpVtbl->put_RelativeTime(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_RelativeTime(This, value) \
    ((This)->lpVtbl->get_RelativeTime(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_put_SystemRelativeTime(This, value) \
    ((This)->lpVtbl->put_SystemRelativeTime(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_SystemRelativeTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeTime(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_put_IsDiscontinuous(This, value) \
    ((This)->lpVtbl->put_IsDiscontinuous(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_IsDiscontinuous(This, value) \
    ((This)->lpVtbl->get_IsDiscontinuous(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaFrame_get_ExtendedProperties(This, value) \
    ((This)->lpVtbl->get_ExtendedProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaMarker
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaMarker_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaMarker_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaMarker[] = L"Windows.Media.IMediaMarker";
typedef struct __x_ABI_CWindows_CMedia_CIMediaMarkerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaMarker* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaMarker* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Time)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaMarkerType)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Text)(__x_ABI_CWindows_CMedia_CIMediaMarker* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaMarkerVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaMarker
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaMarkerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaMarker_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_get_Time(This, value) \
    ((This)->lpVtbl->get_Time(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_get_MediaMarkerType(This, value) \
    ((This)->lpVtbl->get_MediaMarkerType(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaMarker_get_Text(This, value) \
    ((This)->lpVtbl->get_Text(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaMarker;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaMarker_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaMarkerTypesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaMarkerTypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaMarkerTypesStatics[] = L"Windows.Media.IMediaMarkerTypesStatics";
typedef struct __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bookmark)(__x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_get_Bookmark(This, value) \
    ((This)->lpVtbl->get_Bookmark(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaMarkerTypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaMarkers
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaMarkers_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaMarkers_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaMarkers[] = L"Windows.Media.IMediaMarkers";
typedef struct __x_ABI_CWindows_CMedia_CIMediaMarkersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Markers)(__x_ABI_CWindows_CMedia_CIMediaMarkers* This,
        __FIVectorView_1_Windows__CMedia__CIMediaMarker** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaMarkersVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaMarkers
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaMarkersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaMarkers_get_Markers(This, value) \
    ((This)->lpVtbl->get_Markers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaMarkers;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaMarkers_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaProcessingTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProcessingTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaProcessingTriggerDetails[] = L"Windows.Media.IMediaProcessingTriggerDetails";
typedef struct __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Arguments)(__x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetailsVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_get_Arguments(This, value) \
    ((This)->lpVtbl->get_Arguments(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaProcessingTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMediaTimelineController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaTimelineController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaTimelineController[] = L"Windows.Media.IMediaTimelineController";
typedef struct __x_ABI_CWindows_CMedia_CIMediaTimelineControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This);
    HRESULT (STDMETHODCALLTYPE* Resume)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Position)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_ClockRate)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_ClockRate)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        enum __x_ABI_CWindows_CMedia_CMediaTimelineControllerState* value);
    HRESULT (STDMETHODCALLTYPE* add_PositionChanged)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* positionChangedEventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_PositionChanged)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* stateChangedEventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CMedia_CIMediaTimelineController* This,
        EventRegistrationToken eventCookie);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaTimelineControllerVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaTimelineController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaTimelineControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_Resume(This) \
    ((This)->lpVtbl->Resume(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_put_Position(This, value) \
    ((This)->lpVtbl->put_Position(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_get_ClockRate(This, value) \
    ((This)->lpVtbl->get_ClockRate(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_put_ClockRate(This, value) \
    ((This)->lpVtbl->put_ClockRate(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_add_PositionChanged(This, positionChangedEventHandler, eventCookie) \
    ((This)->lpVtbl->add_PositionChanged(This, positionChangedEventHandler, eventCookie))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_remove_PositionChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_PositionChanged(This, eventCookie))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_add_StateChanged(This, stateChangedEventHandler, eventCookie) \
    ((This)->lpVtbl->add_StateChanged(This, stateChangedEventHandler, eventCookie))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController_remove_StateChanged(This, eventCookie) \
    ((This)->lpVtbl->remove_StateChanged(This, eventCookie))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaTimelineController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.IMediaTimelineController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaTimelineController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaTimelineController2[] = L"Windows.Media.IMediaTimelineController2";
typedef struct __x_ABI_CWindows_CMedia_CIMediaTimelineController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_Duration)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_IsLoopingEnabled)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsLoopingEnabled)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* add_Failed)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_Windows__CMedia__CMediaTimelineControllerFailedEventArgs* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Failed)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_Ended)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        __FITypedEventHandler_2_Windows__CMedia__CMediaTimelineController_IInspectable* eventHandler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_Ended)(__x_ABI_CWindows_CMedia_CIMediaTimelineController2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaTimelineController2Vtbl;

interface __x_ABI_CWindows_CMedia_CIMediaTimelineController2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaTimelineController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_put_Duration(This, value) \
    ((This)->lpVtbl->put_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_get_IsLoopingEnabled(This, value) \
    ((This)->lpVtbl->get_IsLoopingEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_put_IsLoopingEnabled(This, value) \
    ((This)->lpVtbl->put_IsLoopingEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_add_Failed(This, eventHandler, token) \
    ((This)->lpVtbl->add_Failed(This, eventHandler, token))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_remove_Failed(This, token) \
    ((This)->lpVtbl->remove_Failed(This, token))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_add_Ended(This, eventHandler, token) \
    ((This)->lpVtbl->add_Ended(This, eventHandler, token))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineController2_remove_Ended(This, token) \
    ((This)->lpVtbl->remove_Ended(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaTimelineController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IMediaTimelineControllerFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaTimelineControllerFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMediaTimelineControllerFailedEventArgs[] = L"Windows.Media.IMediaTimelineControllerFailedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMediaTimelineControllerFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.IMusicDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MusicDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMusicDisplayProperties[] = L"Windows.Media.IMusicDisplayProperties";
typedef struct __x_ABI_CWindows_CMedia_CIMusicDisplayPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_AlbumArtist)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AlbumArtist)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Artist)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Artist)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMusicDisplayPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CIMusicDisplayProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMusicDisplayPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_get_AlbumArtist(This, value) \
    ((This)->lpVtbl->get_AlbumArtist(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_put_AlbumArtist(This, value) \
    ((This)->lpVtbl->put_AlbumArtist(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_get_Artist(This, value) \
    ((This)->lpVtbl->get_Artist(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties_put_Artist(This, value) \
    ((This)->lpVtbl->put_Artist(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMusicDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMusicDisplayProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MusicDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMusicDisplayProperties2[] = L"Windows.Media.IMusicDisplayProperties2";
typedef struct __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlbumTitle)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AlbumTitle)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_TrackNumber)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_TrackNumber)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Genres)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties2* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2Vtbl;

interface __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_get_AlbumTitle(This, value) \
    ((This)->lpVtbl->get_AlbumTitle(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_put_AlbumTitle(This, value) \
    ((This)->lpVtbl->put_AlbumTitle(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_get_TrackNumber(This, value) \
    ((This)->lpVtbl->get_TrackNumber(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_put_TrackNumber(This, value) \
    ((This)->lpVtbl->put_TrackNumber(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_get_Genres(This, value) \
    ((This)->lpVtbl->get_Genres(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMusicDisplayProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IMusicDisplayProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.MusicDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IMusicDisplayProperties3[] = L"Windows.Media.IMusicDisplayProperties3";
typedef struct __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AlbumTrackCount)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_AlbumTrackCount)(__x_ABI_CWindows_CMedia_CIMusicDisplayProperties3* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3Vtbl;

interface __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_get_AlbumTrackCount(This, value) \
    ((This)->lpVtbl->get_AlbumTrackCount(This, value))

#define __x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_put_AlbumTrackCount(This, value) \
    ((This)->lpVtbl->put_AlbumTrackCount(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIMusicDisplayProperties3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIMusicDisplayProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.IPlaybackPositionChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlaybackPositionChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IPlaybackPositionChangeRequestedEventArgs[] = L"Windows.Media.IPlaybackPositionChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestedPlaybackPosition)(__x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_get_RequestedPlaybackPosition(This, value) \
    ((This)->lpVtbl->get_RequestedPlaybackPosition(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIPlaybackPositionChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IPlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.PlaybackRateChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IPlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.IPlaybackRateChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestedPlaybackRate)(__x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_get_RequestedPlaybackRate(This, value) \
    ((This)->lpVtbl->get_RequestedPlaybackRate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIPlaybackRateChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IShuffleEnabledChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.ShuffleEnabledChangeRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IShuffleEnabledChangeRequestedEventArgs[] = L"Windows.Media.IShuffleEnabledChangeRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestedShuffleEnabled)(__x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_get_RequestedShuffleEnabled(This, value) \
    ((This)->lpVtbl->get_RequestedShuffleEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIShuffleEnabledChangeRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControls
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControls
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControls[] = L"Windows.Media.ISystemMediaTransportControls";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PlaybackStatus)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackStatus* value);
    HRESULT (STDMETHODCALLTYPE* put_PlaybackStatus)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackStatus value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayUpdater)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater** value);
    HRESULT (STDMETHODCALLTYPE* get_SoundLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        enum __x_ABI_CWindows_CMedia_CSoundLevel* value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsPlayEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPlayEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsStopEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsStopEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsPauseEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPauseEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsRecordEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsRecordEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsFastForwardEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsFastForwardEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsRewindEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsRewindEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsPreviousEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsPreviousEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsNextEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsNextEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsChannelUpEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsChannelUpEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsChannelDownEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsChannelDownEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* add_ButtonPressed)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsButtonPressedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ButtonPressed)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PropertyChanged)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CSystemMediaTransportControlsPropertyChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PropertyChanged)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsVtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControls
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_PlaybackStatus(This, value) \
    ((This)->lpVtbl->get_PlaybackStatus(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_PlaybackStatus(This, value) \
    ((This)->lpVtbl->put_PlaybackStatus(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_DisplayUpdater(This, value) \
    ((This)->lpVtbl->get_DisplayUpdater(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_SoundLevel(This, value) \
    ((This)->lpVtbl->get_SoundLevel(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsEnabled(This, value) \
    ((This)->lpVtbl->put_IsEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsPlayEnabled(This, value) \
    ((This)->lpVtbl->get_IsPlayEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsPlayEnabled(This, value) \
    ((This)->lpVtbl->put_IsPlayEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsStopEnabled(This, value) \
    ((This)->lpVtbl->get_IsStopEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsStopEnabled(This, value) \
    ((This)->lpVtbl->put_IsStopEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsPauseEnabled(This, value) \
    ((This)->lpVtbl->get_IsPauseEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsPauseEnabled(This, value) \
    ((This)->lpVtbl->put_IsPauseEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsRecordEnabled(This, value) \
    ((This)->lpVtbl->get_IsRecordEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsRecordEnabled(This, value) \
    ((This)->lpVtbl->put_IsRecordEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsFastForwardEnabled(This, value) \
    ((This)->lpVtbl->get_IsFastForwardEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsFastForwardEnabled(This, value) \
    ((This)->lpVtbl->put_IsFastForwardEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsRewindEnabled(This, value) \
    ((This)->lpVtbl->get_IsRewindEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsRewindEnabled(This, value) \
    ((This)->lpVtbl->put_IsRewindEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsPreviousEnabled(This, value) \
    ((This)->lpVtbl->get_IsPreviousEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsPreviousEnabled(This, value) \
    ((This)->lpVtbl->put_IsPreviousEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsNextEnabled(This, value) \
    ((This)->lpVtbl->get_IsNextEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsNextEnabled(This, value) \
    ((This)->lpVtbl->put_IsNextEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsChannelUpEnabled(This, value) \
    ((This)->lpVtbl->get_IsChannelUpEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsChannelUpEnabled(This, value) \
    ((This)->lpVtbl->put_IsChannelUpEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_get_IsChannelDownEnabled(This, value) \
    ((This)->lpVtbl->get_IsChannelDownEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_put_IsChannelDownEnabled(This, value) \
    ((This)->lpVtbl->put_IsChannelDownEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_add_ButtonPressed(This, handler, token) \
    ((This)->lpVtbl->add_ButtonPressed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_remove_ButtonPressed(This, token) \
    ((This)->lpVtbl->remove_ButtonPressed(This, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_add_PropertyChanged(This, handler, token) \
    ((This)->lpVtbl->add_PropertyChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls_remove_PropertyChanged(This, token) \
    ((This)->lpVtbl->remove_PropertyChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControls;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControls2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControls
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControls2[] = L"Windows.Media.ISystemMediaTransportControls2";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AutoRepeatMode)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackAutoRepeatMode* value);
    HRESULT (STDMETHODCALLTYPE* put_AutoRepeatMode)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackAutoRepeatMode value);
    HRESULT (STDMETHODCALLTYPE* get_ShuffleEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ShuffleEnabled)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PlaybackRate)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_PlaybackRate)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* UpdateTimelineProperties)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* timelineProperties);
    HRESULT (STDMETHODCALLTYPE* add_PlaybackPositionChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackPositionChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PlaybackPositionChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PlaybackRateChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CPlaybackRateChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PlaybackRateChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ShuffleEnabledChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CShuffleEnabledChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ShuffleEnabledChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AutoRepeatModeChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        __FITypedEventHandler_2_Windows__CMedia__CSystemMediaTransportControls_Windows__CMedia__CAutoRepeatModeChangeRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AutoRepeatModeChangeRequested)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControls2* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2Vtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_get_AutoRepeatMode(This, value) \
    ((This)->lpVtbl->get_AutoRepeatMode(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_put_AutoRepeatMode(This, value) \
    ((This)->lpVtbl->put_AutoRepeatMode(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_get_ShuffleEnabled(This, value) \
    ((This)->lpVtbl->get_ShuffleEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_put_ShuffleEnabled(This, value) \
    ((This)->lpVtbl->put_ShuffleEnabled(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_get_PlaybackRate(This, value) \
    ((This)->lpVtbl->get_PlaybackRate(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_put_PlaybackRate(This, value) \
    ((This)->lpVtbl->put_PlaybackRate(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_UpdateTimelineProperties(This, timelineProperties) \
    ((This)->lpVtbl->UpdateTimelineProperties(This, timelineProperties))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_add_PlaybackPositionChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_PlaybackPositionChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_remove_PlaybackPositionChangeRequested(This, token) \
    ((This)->lpVtbl->remove_PlaybackPositionChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_add_PlaybackRateChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_PlaybackRateChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_remove_PlaybackRateChangeRequested(This, token) \
    ((This)->lpVtbl->remove_PlaybackRateChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_add_ShuffleEnabledChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_ShuffleEnabledChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_remove_ShuffleEnabledChangeRequested(This, token) \
    ((This)->lpVtbl->remove_ShuffleEnabledChangeRequested(This, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_add_AutoRepeatModeChangeRequested(This, handler, token) \
    ((This)->lpVtbl->add_AutoRepeatModeChangeRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_remove_AutoRepeatModeChangeRequested(This, token) \
    ((This)->lpVtbl->remove_AutoRepeatModeChangeRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControls2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControls2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsButtonPressedEventArgs[] = L"Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Button)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsButton* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_get_Button(This, value) \
    ((This)->lpVtbl->get_Button(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsButtonPressedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsDisplayUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsDisplayUpdater
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsDisplayUpdater[] = L"Windows.Media.ISystemMediaTransportControlsDisplayUpdater";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdaterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackType* value);
    HRESULT (STDMETHODCALLTYPE* put_Type)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackType value);
    HRESULT (STDMETHODCALLTYPE* get_AppMediaId)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_AppMediaId)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Thumbnail)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);
    HRESULT (STDMETHODCALLTYPE* put_Thumbnail)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference* value);
    HRESULT (STDMETHODCALLTYPE* get_MusicProperties)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        __x_ABI_CWindows_CMedia_CIMusicDisplayProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoProperties)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        __x_ABI_CWindows_CMedia_CIVideoDisplayProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_ImageProperties)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        __x_ABI_CWindows_CMedia_CIImageDisplayProperties** value);
    HRESULT (STDMETHODCALLTYPE* CopyFromFileAsync)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This,
        enum __x_ABI_CWindows_CMedia_CMediaPlaybackType type,
        __x_ABI_CWindows_CStorage_CIStorageFile* source,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* ClearAll)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This);
    HRESULT (STDMETHODCALLTYPE* Update)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdaterVtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdaterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_put_Type(This, value) \
    ((This)->lpVtbl->put_Type(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_get_AppMediaId(This, value) \
    ((This)->lpVtbl->get_AppMediaId(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_put_AppMediaId(This, value) \
    ((This)->lpVtbl->put_AppMediaId(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_get_Thumbnail(This, value) \
    ((This)->lpVtbl->get_Thumbnail(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_put_Thumbnail(This, value) \
    ((This)->lpVtbl->put_Thumbnail(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_get_MusicProperties(This, value) \
    ((This)->lpVtbl->get_MusicProperties(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_get_VideoProperties(This, value) \
    ((This)->lpVtbl->get_VideoProperties(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_get_ImageProperties(This, value) \
    ((This)->lpVtbl->get_ImageProperties(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_CopyFromFileAsync(This, type, source, operation) \
    ((This)->lpVtbl->CopyFromFileAsync(This, type, source, operation))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_ClearAll(This) \
    ((This)->lpVtbl->ClearAll(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_Update(This) \
    ((This)->lpVtbl->Update(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsDisplayUpdater_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsPropertyChangedEventArgs[] = L"Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Property)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CSystemMediaTransportControlsProperty* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_get_Property(This, value) \
    ((This)->lpVtbl->get_Property(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsPropertyChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControls
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsStatics[] = L"Windows.Media.ISystemMediaTransportControlsStatics";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentView)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics* This,
        __x_ABI_CWindows_CMedia_CISystemMediaTransportControls** mediaControl);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_GetForCurrentView(This, mediaControl) \
    ((This)->lpVtbl->GetForCurrentView(This, mediaControl))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.ISystemMediaTransportControlsTimelineProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.SystemMediaTransportControlsTimelineProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_ISystemMediaTransportControlsTimelineProperties[] = L"Windows.Media.ISystemMediaTransportControlsTimelineProperties";
typedef struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelinePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_EndTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_EndTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_MinSeekTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_MinSeekTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_MaxSeekTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_MaxSeekTime)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_Position)(__x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelinePropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelinePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_get_EndTime(This, value) \
    ((This)->lpVtbl->get_EndTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_put_EndTime(This, value) \
    ((This)->lpVtbl->put_EndTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_get_MinSeekTime(This, value) \
    ((This)->lpVtbl->get_MinSeekTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_put_MinSeekTime(This, value) \
    ((This)->lpVtbl->put_MinSeekTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_get_MaxSeekTime(This, value) \
    ((This)->lpVtbl->get_MaxSeekTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_put_MaxSeekTime(This, value) \
    ((This)->lpVtbl->put_MaxSeekTime(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_put_Position(This, value) \
    ((This)->lpVtbl->put_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CISystemMediaTransportControlsTimelineProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoDisplayProperties[] = L"Windows.Media.IVideoDisplayProperties";
typedef struct __x_ABI_CWindows_CMedia_CIVideoDisplayPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Subtitle)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subtitle)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoDisplayPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CIVideoDisplayProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoDisplayPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_get_Subtitle(This, value) \
    ((This)->lpVtbl->get_Subtitle(This, value))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties_put_Subtitle(This, value) \
    ((This)->lpVtbl->put_Subtitle(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoDisplayProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoDisplayProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoDisplayProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoDisplayProperties2[] = L"Windows.Media.IVideoDisplayProperties2";
typedef struct __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Genres)(__x_ABI_CWindows_CMedia_CIVideoDisplayProperties2* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2Vtbl;

interface __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_get_Genres(This, value) \
    ((This)->lpVtbl->get_Genres(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoDisplayProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoDisplayProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoEffectsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoEffects
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoEffectsStatics[] = L"Windows.Media.IVideoEffectsStatics";
typedef struct __x_ABI_CWindows_CMedia_CIVideoEffectsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VideoStabilization)(__x_ABI_CWindows_CMedia_CIVideoEffectsStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoEffectsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CIVideoEffectsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoEffectsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoEffectsStatics_get_VideoStabilization(This, value) \
    ((This)->lpVtbl->get_VideoStabilization(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoEffectsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoEffectsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.IMediaFrame
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrame[] = L"Windows.Media.IVideoFrame";
typedef struct __x_ABI_CWindows_CMedia_CIVideoFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SoftwareBitmap)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* CopyToAsync)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame* frame,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* get_Direct3DSurface)(__x_ABI_CWindows_CMedia_CIVideoFrame* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoFrameVtbl;

interface __x_ABI_CWindows_CMedia_CIVideoFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_get_SoftwareBitmap(This, value) \
    ((This)->lpVtbl->get_SoftwareBitmap(This, value))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_CopyToAsync(This, frame, value) \
    ((This)->lpVtbl->CopyToAsync(This, frame, value))

#define __x_ABI_CWindows_CMedia_CIVideoFrame_get_Direct3DSurface(This, value) \
    ((This)->lpVtbl->get_Direct3DSurface(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrame2[] = L"Windows.Media.IVideoFrame2";
typedef struct __x_ABI_CWindows_CMedia_CIVideoFrame2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CopyToWithBoundsAsync)(__x_ABI_CWindows_CMedia_CIVideoFrame2* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame* frame,
        __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* sourceBounds,
        __FIReference_1_Windows__CGraphics__CImaging__CBitmapBounds* destinationBounds,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoFrame2Vtbl;

interface __x_ABI_CWindows_CMedia_CIVideoFrame2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoFrame2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoFrame2_CopyToWithBoundsAsync(This, frame, sourceBounds, destinationBounds, operation) \
    ((This)->lpVtbl->CopyToWithBoundsAsync(This, frame, sourceBounds, destinationBounds, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrame2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.IVideoFrameFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrameFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrameFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrameFactory[] = L"Windows.Media.IVideoFrameFactory";
typedef struct __x_ABI_CWindows_CMedia_CIVideoFrameFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        INT32 width,
        INT32 height,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithAlpha)(__x_ABI_CWindows_CMedia_CIVideoFrameFactory* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat format,
        INT32 width,
        INT32 height,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapAlphaMode alpha,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoFrameFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CIVideoFrameFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoFrameFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_Create(This, format, width, height, value) \
    ((This)->lpVtbl->Create(This, format, width, height, value))

#define __x_ABI_CWindows_CMedia_CIVideoFrameFactory_CreateWithAlpha(This, format, width, height, alpha, value) \
    ((This)->lpVtbl->CreateWithAlpha(This, format, width, height, alpha, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrameFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrameFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.IVideoFrameStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.VideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CIVideoFrameStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CIVideoFrameStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_IVideoFrameStatics[] = L"Windows.Media.IVideoFrameStatics";
typedef struct __x_ABI_CWindows_CMedia_CIVideoFrameStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAsDirect3D11SurfaceBacked)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat format,
        INT32 width,
        INT32 height,
        __x_ABI_CWindows_CMedia_CIVideoFrame** result);
    HRESULT (STDMETHODCALLTYPE* CreateAsDirect3D11SurfaceBackedWithDevice)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        enum __x_ABI_CWindows_CGraphics_CDirectX_CDirectXPixelFormat format,
        INT32 width,
        INT32 height,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DDevice* device,
        __x_ABI_CWindows_CMedia_CIVideoFrame** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithSoftwareBitmap)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap* bitmap,
        __x_ABI_CWindows_CMedia_CIVideoFrame** result);
    HRESULT (STDMETHODCALLTYPE* CreateWithDirect3D11Surface)(__x_ABI_CWindows_CMedia_CIVideoFrameStatics* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface* surface,
        __x_ABI_CWindows_CMedia_CIVideoFrame** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CIVideoFrameStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CIVideoFrameStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CIVideoFrameStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_CreateAsDirect3D11SurfaceBacked(This, format, width, height, result) \
    ((This)->lpVtbl->CreateAsDirect3D11SurfaceBacked(This, format, width, height, result))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_CreateAsDirect3D11SurfaceBackedWithDevice(This, format, width, height, device, result) \
    ((This)->lpVtbl->CreateAsDirect3D11SurfaceBackedWithDevice(This, format, width, height, device, result))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_CreateWithSoftwareBitmap(This, bitmap, result) \
    ((This)->lpVtbl->CreateWithSoftwareBitmap(This, bitmap, result))

#define __x_ABI_CWindows_CMedia_CIVideoFrameStatics_CreateWithDirect3D11Surface(This, surface, result) \
    ((This)->lpVtbl->CreateWithDirect3D11Surface(This, surface, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CIVideoFrameStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CIVideoFrameStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.AudioBuffer
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IAudioBuffer ** Default Interface **
 *    Windows.Foundation.IMemoryBuffer
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AudioBuffer_DEFINED
#define RUNTIMECLASS_Windows_Media_AudioBuffer_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AudioBuffer[] = L"Windows.Media.AudioBuffer";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AudioFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.IAudioFrameFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IAudioFrame ** Default Interface **
 *    Windows.Media.IMediaFrame
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AudioFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_AudioFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AudioFrame[] = L"Windows.Media.AudioFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.AutoRepeatModeChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IAutoRepeatModeChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_AutoRepeatModeChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_AutoRepeatModeChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_AutoRepeatModeChangeRequestedEventArgs[] = L"Windows.Media.AutoRepeatModeChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ImageDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IImageDisplayProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ImageDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_ImageDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ImageDisplayProperties[] = L"Windows.Media.ImageDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaControl
 *
 * Introduced to Windows.Media.MediaControlContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IMediaControl interface starting with version 1.0 of the Windows.Media.MediaControlContract API contract
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaControl_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaControl_DEFINED
#if WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
DEPRECATED("MediaControl may be altered or unavailable for releases after Windows 8.1. Instead, use SystemMediaTransportControls.")
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaControl[] = L"Windows.Media.MediaControl";
#endif
#endif // WINDOWS_MEDIA_MEDIACONTROLCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaExtensionManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaExtensionManager ** Default Interface **
 *    Windows.Media.IMediaExtensionManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaExtensionManager_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaExtensionManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaExtensionManager[] = L"Windows.Media.MediaExtensionManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaMarkerTypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IMediaMarkerTypesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaMarkerTypes_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaMarkerTypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaMarkerTypes[] = L"Windows.Media.MediaMarkerTypes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProcessingTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaProcessingTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProcessingTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProcessingTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProcessingTriggerDetails[] = L"Windows.Media.MediaProcessingTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaTimelineController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaTimelineController ** Default Interface **
 *    Windows.Media.IMediaTimelineController2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_MediaTimelineController_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaTimelineController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaTimelineController[] = L"Windows.Media.MediaTimelineController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.MediaTimelineControllerFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMediaTimelineControllerFailedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_MediaTimelineControllerFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaTimelineControllerFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaTimelineControllerFailedEventArgs[] = L"Windows.Media.MediaTimelineControllerFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.MusicDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IMusicDisplayProperties ** Default Interface **
 *    Windows.Media.IMusicDisplayProperties2
 *    Windows.Media.IMusicDisplayProperties3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MusicDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MusicDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MusicDisplayProperties[] = L"Windows.Media.MusicDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlaybackPositionChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IPlaybackPositionChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlaybackPositionChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlaybackPositionChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlaybackPositionChangeRequestedEventArgs[] = L"Windows.Media.PlaybackPositionChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.PlaybackRateChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IPlaybackRateChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_PlaybackRateChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_PlaybackRateChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_PlaybackRateChangeRequestedEventArgs[] = L"Windows.Media.PlaybackRateChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.ShuffleEnabledChangeRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IShuffleEnabledChangeRequestedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_ShuffleEnabledChangeRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_ShuffleEnabledChangeRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_ShuffleEnabledChangeRequestedEventArgs[] = L"Windows.Media.ShuffleEnabledChangeRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControls
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.ISystemMediaTransportControlsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControls ** Default Interface **
 *    Windows.Media.ISystemMediaTransportControls2
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControls_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControls_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControls[] = L"Windows.Media.SystemMediaTransportControls";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsButtonPressedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsButtonPressedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsButtonPressedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsButtonPressedEventArgs[] = L"Windows.Media.SystemMediaTransportControlsButtonPressedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsDisplayUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsDisplayUpdater ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsDisplayUpdater_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsDisplayUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsDisplayUpdater[] = L"Windows.Media.SystemMediaTransportControlsDisplayUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsPropertyChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsPropertyChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsPropertyChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsPropertyChangedEventArgs[] = L"Windows.Media.SystemMediaTransportControlsPropertyChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.SystemMediaTransportControlsTimelineProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.ISystemMediaTransportControlsTimelineProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsTimelineProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_SystemMediaTransportControlsTimelineProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_SystemMediaTransportControlsTimelineProperties[] = L"Windows.Media.SystemMediaTransportControlsTimelineProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.VideoDisplayProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.IVideoDisplayProperties ** Default Interface **
 *    Windows.Media.IVideoDisplayProperties2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_VideoDisplayProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_VideoDisplayProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_VideoDisplayProperties[] = L"Windows.Media.VideoDisplayProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.VideoEffects
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IVideoEffectsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_VideoEffects_DEFINED
#define RUNTIMECLASS_Windows_Media_VideoEffects_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_VideoEffects[] = L"Windows.Media.VideoEffects";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.VideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.IVideoFrameFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.IVideoFrameStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.IVideoFrame ** Default Interface **
 *    Windows.Media.IMediaFrame
 *    Windows.Foundation.IClosable
 *    Windows.Media.IVideoFrame2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_VideoFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_VideoFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_VideoFrame[] = L"Windows.Media.VideoFrame";
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
#endif // __windows2Emedia_p_h__

#endif // __windows2Emedia_h__
