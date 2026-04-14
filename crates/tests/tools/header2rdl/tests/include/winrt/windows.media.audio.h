
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
#ifndef __windows2Emedia2Eaudio_h__
#define __windows2Emedia2Eaudio_h__
#ifndef __windows2Emedia2Eaudio_p_h__
#define __windows2Emedia2Eaudio_p_h__


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
#include "Windows.Foundation.Numerics.h"
#include "Windows.Media.h"
#include "Windows.Media.Capture.h"
#include "Windows.Media.Core.h"
#include "Windows.Media.Devices.h"
#include "Windows.Media.Effects.h"
#include "Windows.Media.MediaProperties.h"
#include "Windows.Media.Render.h"
#include "Windows.Media.Transcoding.h"
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioDeviceInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode ABI::Windows::Media::Audio::IAudioDeviceInputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioDeviceOutputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode ABI::Windows::Media::Audio::IAudioDeviceOutputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioEffectsPackConfiguration;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration ABI::Windows::Media::Audio::IAudioEffectsPackConfiguration

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioEffectsPackConfigurationStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics ABI::Windows::Media::Audio::IAudioEffectsPackConfigurationStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioFileInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode ABI::Windows::Media::Audio::IAudioFileInputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioFileOutputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode ABI::Windows::Media::Audio::IAudioFileOutputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioFrameCompletedEventArgs;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs ABI::Windows::Media::Audio::IAudioFrameCompletedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioFrameInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode ABI::Windows::Media::Audio::IAudioFrameInputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioFrameOutputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode ABI::Windows::Media::Audio::IAudioFrameOutputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraph;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph ABI::Windows::Media::Audio::IAudioGraph

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraph2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2 ABI::Windows::Media::Audio::IAudioGraph2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraph3;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3 ABI::Windows::Media::Audio::IAudioGraph3

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraphConnection;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection ABI::Windows::Media::Audio::IAudioGraphConnection

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraphSettings;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings ABI::Windows::Media::Audio::IAudioGraphSettings

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraphSettings2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2 ABI::Windows::Media::Audio::IAudioGraphSettings2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraphSettingsFactory;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory ABI::Windows::Media::Audio::IAudioGraphSettingsFactory

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraphStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics ABI::Windows::Media::Audio::IAudioGraphStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioGraphUnrecoverableErrorOccurredEventArgs;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs ABI::Windows::Media::Audio::IAudioGraphUnrecoverableErrorOccurredEventArgs

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode ABI::Windows::Media::Audio::IAudioInputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioInputNode2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2 ABI::Windows::Media::Audio::IAudioInputNode2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode ABI::Windows::Media::Audio::IAudioNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitter;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter ABI::Windows::Media::Audio::IAudioNodeEmitter

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitter2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2 ABI::Windows::Media::Audio::IAudioNodeEmitter2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterConeProperties;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties ABI::Windows::Media::Audio::IAudioNodeEmitterConeProperties

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterDecayModel;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel ABI::Windows::Media::Audio::IAudioNodeEmitterDecayModel

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterDecayModelStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics ABI::Windows::Media::Audio::IAudioNodeEmitterDecayModelStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterFactory;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory ABI::Windows::Media::Audio::IAudioNodeEmitterFactory

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterNaturalDecayModelProperties;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties ABI::Windows::Media::Audio::IAudioNodeEmitterNaturalDecayModelProperties

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterShape;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape ABI::Windows::Media::Audio::IAudioNodeEmitterShape

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeEmitterShapeStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics ABI::Windows::Media::Audio::IAudioNodeEmitterShapeStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeListener;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener ABI::Windows::Media::Audio::IAudioNodeListener

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioNodeWithListener;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener ABI::Windows::Media::Audio::IAudioNodeWithListener

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioPlaybackConnection;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection ABI::Windows::Media::Audio::IAudioPlaybackConnection

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioPlaybackConnectionOpenResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult ABI::Windows::Media::Audio::IAudioPlaybackConnectionOpenResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioPlaybackConnectionStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics ABI::Windows::Media::Audio::IAudioPlaybackConnectionStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioStateMonitor;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor ABI::Windows::Media::Audio::IAudioStateMonitor

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IAudioStateMonitorStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics ABI::Windows::Media::Audio::IAudioStateMonitorStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioDeviceInputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult ABI::Windows::Media::Audio::ICreateAudioDeviceInputNodeResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioDeviceInputNodeResult2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2 ABI::Windows::Media::Audio::ICreateAudioDeviceInputNodeResult2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioDeviceOutputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult ABI::Windows::Media::Audio::ICreateAudioDeviceOutputNodeResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioDeviceOutputNodeResult2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2 ABI::Windows::Media::Audio::ICreateAudioDeviceOutputNodeResult2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioFileInputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult ABI::Windows::Media::Audio::ICreateAudioFileInputNodeResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioFileInputNodeResult2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2 ABI::Windows::Media::Audio::ICreateAudioFileInputNodeResult2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioFileOutputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult ABI::Windows::Media::Audio::ICreateAudioFileOutputNodeResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioFileOutputNodeResult2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2 ABI::Windows::Media::Audio::ICreateAudioFileOutputNodeResult2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioGraphResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult ABI::Windows::Media::Audio::ICreateAudioGraphResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateAudioGraphResult2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2 ABI::Windows::Media::Audio::ICreateAudioGraphResult2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateMediaSourceAudioInputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult ABI::Windows::Media::Audio::ICreateMediaSourceAudioInputNodeResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ICreateMediaSourceAudioInputNodeResult2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2 ABI::Windows::Media::Audio::ICreateMediaSourceAudioInputNodeResult2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IEchoEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition ABI::Windows::Media::Audio::IEchoEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IEchoEffectDefinitionFactory;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory ABI::Windows::Media::Audio::IEchoEffectDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IEqualizerBand;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand ABI::Windows::Media::Audio::IEqualizerBand

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IEqualizerEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition ABI::Windows::Media::Audio::IEqualizerEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IEqualizerEffectDefinitionFactory;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory ABI::Windows::Media::Audio::IEqualizerEffectDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IFrameInputNodeQuantumStartedEventArgs;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs ABI::Windows::Media::Audio::IFrameInputNodeQuantumStartedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ILimiterEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition ABI::Windows::Media::Audio::ILimiterEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ILimiterEffectDefinitionFactory;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory ABI::Windows::Media::Audio::ILimiterEffectDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IMediaSourceAudioInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode ABI::Windows::Media::Audio::IMediaSourceAudioInputNode

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IReverbEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition ABI::Windows::Media::Audio::IReverbEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface IReverbEffectDefinitionFactory;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory ABI::Windows::Media::Audio::IReverbEffectDefinitionFactory

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISetDefaultSpatialAudioFormatResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult ABI::Windows::Media::Audio::ISetDefaultSpatialAudioFormatResult

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISpatialAudioDeviceConfiguration;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration ABI::Windows::Media::Audio::ISpatialAudioDeviceConfiguration

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISpatialAudioDeviceConfigurationStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics ABI::Windows::Media::Audio::ISpatialAudioDeviceConfigurationStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISpatialAudioFormatConfiguration;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration ABI::Windows::Media::Audio::ISpatialAudioFormatConfiguration

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISpatialAudioFormatConfigurationStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics ABI::Windows::Media::Audio::ISpatialAudioFormatConfigurationStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISpatialAudioFormatSubtypeStatics;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics ABI::Windows::Media::Audio::ISpatialAudioFormatSubtypeStatics

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                interface ISpatialAudioFormatSubtypeStatics2;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2 ABI::Windows::Media::Audio::ISpatialAudioFormatSubtypeStatics2

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioPlaybackConnectionOpenResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f5245f8a-3dd1-56b2-829b-9888251d689c"))
IAsyncOperation<ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResult*, ABI::Windows::Media::Audio::IAudioPlaybackConnectionOpenResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.AudioPlaybackConnectionOpenResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("56ddb54d-eb8d-5ffb-a54b-8faf918c8031"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResult*, ABI::Windows::Media::Audio::IAudioPlaybackConnectionOpenResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.AudioPlaybackConnectionOpenResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class CreateAudioDeviceInputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("71ab4481-ec4a-5ee9-a342-3a31747829b8"))
IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioDeviceInputNodeResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioDeviceInputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioDeviceInputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.CreateAudioDeviceInputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioDeviceInputNodeResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6cc56450-e4e8-59c9-83d8-63e46eacb20b"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioDeviceInputNodeResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioDeviceInputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioDeviceInputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.CreateAudioDeviceInputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioDeviceInputNodeResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class CreateAudioDeviceOutputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f810d730-de15-58e0-a5f4-c159f73669ed"))
IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioDeviceOutputNodeResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioDeviceOutputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioDeviceOutputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.CreateAudioDeviceOutputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioDeviceOutputNodeResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("edbc9b59-7cae-513f-b0dc-17666d37ba77"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioDeviceOutputNodeResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioDeviceOutputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioDeviceOutputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.CreateAudioDeviceOutputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioDeviceOutputNodeResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class CreateAudioFileInputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("473b06bf-387b-56ca-bee1-527480272b0f"))
IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioFileInputNodeResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioFileInputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioFileInputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.CreateAudioFileInputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioFileInputNodeResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("504d1efd-c11c-506e-b8c9-af17c771efb5"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioFileInputNodeResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioFileInputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioFileInputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.CreateAudioFileInputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioFileInputNodeResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class CreateAudioFileOutputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1164517d-e953-5415-a5b3-4249a969be7b"))
IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioFileOutputNodeResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioFileOutputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioFileOutputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.CreateAudioFileOutputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioFileOutputNodeResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a7a95713-a08f-5fdf-89c6-9627bcf5d80a"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioFileOutputNodeResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioFileOutputNodeResult*, ABI::Windows::Media::Audio::ICreateAudioFileOutputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.CreateAudioFileOutputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioFileOutputNodeResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class CreateAudioGraphResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e13b431-65ce-5bfb-b0aa-fac8df958b95"))
IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioGraphResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioGraphResult*, ABI::Windows::Media::Audio::ICreateAudioGraphResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.CreateAudioGraphResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::CreateAudioGraphResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4e660bda-d438-5741-8b66-85fe72574aab"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioGraphResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateAudioGraphResult*, ABI::Windows::Media::Audio::ICreateAudioGraphResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.CreateAudioGraphResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateAudioGraphResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class CreateMediaSourceAudioInputNodeResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c69c20f3-88fc-5f3a-95d0-2816eae45968"))
IAsyncOperation<ABI::Windows::Media::Audio::CreateMediaSourceAudioInputNodeResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateMediaSourceAudioInputNodeResult*, ABI::Windows::Media::Audio::ICreateMediaSourceAudioInputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::CreateMediaSourceAudioInputNodeResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e471b446-3a01-5190-89c9-15e740374cfc"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateMediaSourceAudioInputNodeResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::CreateMediaSourceAudioInputNodeResult*, ABI::Windows::Media::Audio::ICreateMediaSourceAudioInputNodeResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::CreateMediaSourceAudioInputNodeResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class SetDefaultSpatialAudioFormatResult;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("49e3b522-42d9-5909-80f1-33f9dae69bc1"))
IAsyncOperation<ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatResult*, ABI::Windows::Media::Audio::ISetDefaultSpatialAudioFormatResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Audio.SetDefaultSpatialAudioFormatResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatResult*> __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b059d94d-0c13-5e6b-9cda-b3cfa15b5be8"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatResult*, ABI::Windows::Media::Audio::ISetDefaultSpatialAudioFormatResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Audio.SetDefaultSpatialAudioFormatResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Transcoding {
                typedef enum TranscodeFailureReason : int TranscodeFailureReason;
            } /* Transcoding */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("02132510-3899-5257-bed9-a43e5149d28c"))
IAsyncOperation<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason> : IAsyncOperation_impl<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Transcoding.TranscodeFailureReason>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason> __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_t;
#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c42ae2bf-e194-5179-b8ad-03b51c04e1da"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Transcoding.TranscodeFailureReason>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Transcoding::TranscodeFailureReason> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioGraphConnection;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE
#define DEF___FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4af6a8fc-e7fb-5957-91c1-2df9600b22eb"))
IIterator<ABI::Windows::Media::Audio::AudioGraphConnection*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioGraphConnection*, ABI::Windows::Media::Audio::IAudioGraphConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Audio.AudioGraphConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Audio::AudioGraphConnection*> __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_t;
#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE
#define DEF___FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("96168d06-a51a-5480-9403-fbd7631e3b3c"))
IIterable<ABI::Windows::Media::Audio::AudioGraphConnection*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioGraphConnection*, ABI::Windows::Media::Audio::IAudioGraphConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Audio.AudioGraphConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Audio::AudioGraphConnection*> __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_t;
#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class EqualizerBand;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_USE
#define DEF___FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("eb4f8b6a-7928-5f2f-b7f2-7b90c084356f"))
IIterator<ABI::Windows::Media::Audio::EqualizerBand*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::EqualizerBand*, ABI::Windows::Media::Audio::IEqualizerBand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Audio.EqualizerBand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Audio::EqualizerBand*> __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_t;
#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_USE
#define DEF___FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6f76d148-023e-565a-9f09-4ad4a32ad74f"))
IIterable<ABI::Windows::Media::Audio::EqualizerBand*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::EqualizerBand*, ABI::Windows::Media::Audio::IEqualizerBand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Audio.EqualizerBand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Audio::EqualizerBand*> __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_t;
#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Effects {
                interface IAudioEffectDefinition;
            } /* Effects */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition ABI::Windows::Media::Effects::IAudioEffectDefinition

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ec0f39fc-6959-5423-9e1a-f7cb8e845ca1"))
IIterator<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IIterator_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("607a20bf-32b4-5b8e-a793-3024f8d3582a"))
IIterable<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IIterable_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE
#define DEF___FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8485aed1-9b0c-59d2-a206-699bf746c3ff"))
IVectorView<ABI::Windows::Media::Audio::AudioGraphConnection*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioGraphConnection*, ABI::Windows::Media::Audio::IAudioGraphConnection*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Audio.AudioGraphConnection>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Audio::AudioGraphConnection*> __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_t;
#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_USE
#define DEF___FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("48f26053-ea7d-59e1-952b-fb78af42d2e2"))
IVectorView<ABI::Windows::Media::Audio::EqualizerBand*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::EqualizerBand*, ABI::Windows::Media::Audio::IEqualizerBand*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Audio.EqualizerBand>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Audio::EqualizerBand*> __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_t;
#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("de9e6a7f-d28e-5ef1-916a-efa880b489d1"))
IVectorView<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IVectorView_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#define DEF___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2828a982-d849-5fc9-84ce-f9a4b3b4d341"))
IVector<ABI::Windows::Media::Effects::IAudioEffectDefinition*> : IVector_impl<ABI::Windows::Media::Effects::IAudioEffectDefinition*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Effects.IAudioEffectDefinition>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Effects::IAudioEffectDefinition*> __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t;
#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIReference_1_int_USE
#define DEF___FIReference_1_int_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("548cefbd-bc8a-5fa0-8df2-957440fc8bf4"))
IReference<int> : IReference_impl<int>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Int32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<int> __FIReference_1_int_t;
#define __FIReference_1_int ABI::Windows::Foundation::__FIReference_1_int_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_int_USE */


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
        namespace Media {
            namespace Audio {
                class AudioEffectsPackConfiguration;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("41cdcf69-8822-5062-af8b-68e32ef9884d"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioEffectsPackConfiguration*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioEffectsPackConfiguration*, ABI::Windows::Media::Audio::IAudioEffectsPackConfiguration*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioEffectsPackConfiguration, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioEffectsPackConfiguration*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioFileInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4481085b-8b8b-5520-9825-e9671da2a89f"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioFileInputNode*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioFileInputNode*, ABI::Windows::Media::Audio::IAudioFileInputNode*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioFileInputNode, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioFileInputNode*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioFrameInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioFrameCompletedEventArgs;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad59dcfe-71b0-5e16-99c2-cd90644d8ee8"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioFrameInputNode*, ABI::Windows::Media::Audio::AudioFrameCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioFrameInputNode*, ABI::Windows::Media::Audio::IAudioFrameInputNode*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioFrameCompletedEventArgs*, ABI::Windows::Media::Audio::IAudioFrameCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioFrameInputNode, Windows.Media.Audio.AudioFrameCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioFrameInputNode*, ABI::Windows::Media::Audio::AudioFrameCompletedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class FrameInputNodeQuantumStartedEventArgs;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4530d121-bb9a-57fe-922f-a98eeedf59af"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioFrameInputNode*, ABI::Windows::Media::Audio::FrameInputNodeQuantumStartedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioFrameInputNode*, ABI::Windows::Media::Audio::IAudioFrameInputNode*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::FrameInputNodeQuantumStartedEventArgs*, ABI::Windows::Media::Audio::IFrameInputNodeQuantumStartedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioFrameInputNode, Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioFrameInputNode*, ABI::Windows::Media::Audio::FrameInputNodeQuantumStartedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioGraph;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e1407134-09e7-53de-b54c-8a0659397b88"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioGraph*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioGraph*, ABI::Windows::Media::Audio::IAudioGraph*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioGraph, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioGraph*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioGraphUnrecoverableErrorOccurredEventArgs;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("899670c9-dd7f-5f12-98cb-8b17fe80a47f"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioGraph*, ABI::Windows::Media::Audio::AudioGraphUnrecoverableErrorOccurredEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioGraph*, ABI::Windows::Media::Audio::IAudioGraph*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioGraphUnrecoverableErrorOccurredEventArgs*, ABI::Windows::Media::Audio::IAudioGraphUnrecoverableErrorOccurredEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioGraph, Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioGraph*, ABI::Windows::Media::Audio::AudioGraphUnrecoverableErrorOccurredEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioPlaybackConnection;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0e389b05-31a6-58f1-9ea4-0c1e4d70a7b8"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioPlaybackConnection*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioPlaybackConnection*, ABI::Windows::Media::Audio::IAudioPlaybackConnection*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioPlaybackConnection, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioPlaybackConnection*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioStateMonitor;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1333df3f-c55b-5a23-9596-34657c2a3406"))
ITypedEventHandler<ABI::Windows::Media::Audio::AudioStateMonitor*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::AudioStateMonitor*, ABI::Windows::Media::Audio::IAudioStateMonitor*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.AudioStateMonitor, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::AudioStateMonitor*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class MediaSourceAudioInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b78980bf-7acf-5dc8-9fcd-31d6ab2f92f1"))
ITypedEventHandler<ABI::Windows::Media::Audio::MediaSourceAudioInputNode*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::MediaSourceAudioInputNode*, ABI::Windows::Media::Audio::IMediaSourceAudioInputNode*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.MediaSourceAudioInputNode, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::MediaSourceAudioInputNode*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class SpatialAudioDeviceConfiguration;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("34cb725d-d620-5c8d-97f6-a3ebdff1f964"))
ITypedEventHandler<ABI::Windows::Media::Audio::SpatialAudioDeviceConfiguration*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Audio::SpatialAudioDeviceConfiguration*, ABI::Windows::Media::Audio::ISpatialAudioDeviceConfiguration*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Audio.SpatialAudioDeviceConfiguration, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Audio::SpatialAudioDeviceConfiguration*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

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
                typedef struct Quaternion Quaternion;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Numerics {
                typedef struct Vector3 Vector3;
            } /* Numerics */
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            class AudioFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Media {
            typedef enum AudioProcessing : int AudioProcessing;
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                typedef enum MediaCategory : int MediaCategory;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                class MediaSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaSource2;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaSource2 ABI::Windows::Media::Core::IMediaSource2

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__

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
            namespace MediaProperties {
                class AudioEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties ABI::Windows::Media::MediaProperties::IAudioEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile ABI::Windows::Media::MediaProperties::IMediaEncodingProfile

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Render {
                typedef enum AudioRenderCategory : int AudioRenderCategory;
            } /* Render */
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
            class StorageFile;
        } /* Storage */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioDeviceNodeCreationStatus : int AudioDeviceNodeCreationStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioEffectsPackStatus : int AudioEffectsPackStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioFileNodeCreationStatus : int AudioFileNodeCreationStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioGraphCreationStatus : int AudioGraphCreationStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioGraphUnrecoverableError : int AudioGraphUnrecoverableError;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioNodeEmitterDecayKind : int AudioNodeEmitterDecayKind;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioNodeEmitterSettings : unsigned int AudioNodeEmitterSettings;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioNodeEmitterShapeKind : int AudioNodeEmitterShapeKind;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioPlaybackConnectionOpenResultStatus : int AudioPlaybackConnectionOpenResultStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum AudioPlaybackConnectionState : int AudioPlaybackConnectionState;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum MediaSourceAudioInputNodeCreationStatus : int MediaSourceAudioInputNodeCreationStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum MixedRealitySpatialAudioFormatPolicy : int MixedRealitySpatialAudioFormatPolicy;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum QuantumSizeSelectionMode : int QuantumSizeSelectionMode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum SetDefaultSpatialAudioFormatStatus : int SetDefaultSpatialAudioFormatStatus;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                typedef enum SpatialAudioModel : int SpatialAudioModel;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioDeviceInputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioDeviceOutputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioFileOutputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioFrameOutputNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioGraphBatchUpdater;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioGraphSettings;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioNodeEmitter;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioNodeEmitterConeProperties;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioNodeEmitterDecayModel;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioNodeEmitterNaturalDecayModelProperties;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioNodeEmitterShape;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioNodeListener;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class AudioSubmixNode;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class EchoEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class EqualizerEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class LimiterEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class ReverbEffectDefinition;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                class SpatialAudioFormatConfiguration;
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Audio.AudioDeviceNodeCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioDeviceNodeCreationStatus : int
                {
                    AudioDeviceNodeCreationStatus_Success = 0,
                    AudioDeviceNodeCreationStatus_DeviceNotAvailable = 1,
                    AudioDeviceNodeCreationStatus_FormatNotSupported = 2,
                    AudioDeviceNodeCreationStatus_UnknownFailure = 3,
                    AudioDeviceNodeCreationStatus_AccessDenied = 4,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioEffectsPackStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioEffectsPackStatus : int
                {
                    AudioEffectsPackStatus_NotEnabled = 0,
                    AudioEffectsPackStatus_Enabled = 1,
                    AudioEffectsPackStatus_NotSupported = 2,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Media.Audio.AudioFileNodeCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioFileNodeCreationStatus : int
                {
                    AudioFileNodeCreationStatus_Success = 0,
                    AudioFileNodeCreationStatus_FileNotFound = 1,
                    AudioFileNodeCreationStatus_InvalidFileType = 2,
                    AudioFileNodeCreationStatus_FormatNotSupported = 3,
                    AudioFileNodeCreationStatus_UnknownFailure = 4,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioGraphCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioGraphCreationStatus : int
                {
                    AudioGraphCreationStatus_Success = 0,
                    AudioGraphCreationStatus_DeviceNotAvailable = 1,
                    AudioGraphCreationStatus_FormatNotSupported = 2,
                    AudioGraphCreationStatus_UnknownFailure = 3,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioGraphUnrecoverableError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioGraphUnrecoverableError : int
                {
                    AudioGraphUnrecoverableError_None = 0,
                    AudioGraphUnrecoverableError_AudioDeviceLost = 1,
                    AudioGraphUnrecoverableError_AudioSessionDisconnected = 2,
                    AudioGraphUnrecoverableError_UnknownFailure = 3,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioNodeEmitterDecayKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioNodeEmitterDecayKind : int
                {
                    AudioNodeEmitterDecayKind_Natural = 0,
                    AudioNodeEmitterDecayKind_Custom = 1,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Audio.AudioNodeEmitterSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioNodeEmitterSettings : unsigned int
                {
                    AudioNodeEmitterSettings_None = 0,
                    AudioNodeEmitterSettings_DisableDoppler = 0x1,
                };

                DEFINE_ENUM_FLAG_OPERATORS(AudioNodeEmitterSettings)
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Audio.AudioNodeEmitterShapeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioNodeEmitterShapeKind : int
                {
                    AudioNodeEmitterShapeKind_Omnidirectional = 0,
                    AudioNodeEmitterShapeKind_Cone = 1,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioPlaybackConnectionOpenResultStatus : int
                {
                    AudioPlaybackConnectionOpenResultStatus_Success = 0,
                    AudioPlaybackConnectionOpenResultStatus_RequestTimedOut = 1,
                    AudioPlaybackConnectionOpenResultStatus_DeniedBySystem = 2,
                    AudioPlaybackConnectionOpenResultStatus_UnknownFailure = 3,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Media.Audio.AudioPlaybackConnectionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum AudioPlaybackConnectionState : int
                {
                    AudioPlaybackConnectionState_Closed = 0,
                    AudioPlaybackConnectionState_Opened = 1,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum MediaSourceAudioInputNodeCreationStatus : int
                {
                    MediaSourceAudioInputNodeCreationStatus_Success = 0,
                    MediaSourceAudioInputNodeCreationStatus_FormatNotSupported = 1,
                    MediaSourceAudioInputNodeCreationStatus_NetworkError = 2,
                    MediaSourceAudioInputNodeCreationStatus_UnknownFailure = 3,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum MixedRealitySpatialAudioFormatPolicy : int
                {
                    MixedRealitySpatialAudioFormatPolicy_UseMixedRealityDefaultSpatialAudioFormat = 0,
                    MixedRealitySpatialAudioFormatPolicy_UseDeviceConfigurationDefaultSpatialAudioFormat = 1,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Media.Audio.QuantumSizeSelectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum QuantumSizeSelectionMode : int
                {
                    QuantumSizeSelectionMode_SystemDefault = 0,
                    QuantumSizeSelectionMode_LowestLatency = 1,
                    QuantumSizeSelectionMode_ClosestToDesired = 2,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum SetDefaultSpatialAudioFormatStatus : int
                {
                    SetDefaultSpatialAudioFormatStatus_Succeeded = 0,
                    SetDefaultSpatialAudioFormatStatus_AccessDenied = 1,
                    SetDefaultSpatialAudioFormatStatus_LicenseExpired = 2,
                    SetDefaultSpatialAudioFormatStatus_LicenseNotValidForAudioEndpoint = 3,
                    SetDefaultSpatialAudioFormatStatus_NotSupportedOnAudioEndpoint = 4,
                    SetDefaultSpatialAudioFormatStatus_UnknownError = 5,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Media.Audio.SpatialAudioModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                enum SpatialAudioModel : int
                {
                    SpatialAudioModel_ObjectBased = 0,
                    SpatialAudioModel_FoldDown = 1,
                };
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioDeviceInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioDeviceInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioDeviceInputNode[] = L"Windows.Media.Audio.IAudioDeviceInputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("b01b6be1-6f4e-49e2-ac01-559d62beb3a9")
                IAudioDeviceInputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Device(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceInputNode = __uuidof(IAudioDeviceInputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioDeviceOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioDeviceOutputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioDeviceOutputNode[] = L"Windows.Media.Audio.IAudioDeviceOutputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("362edbff-ff1c-4434-9e0f-bd2ef522ac82")
                IAudioDeviceOutputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Device(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioDeviceOutputNode = __uuidof(IAudioDeviceOutputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioEffectsPackConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioEffectsPackConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioEffectsPackConfiguration[] = L"Windows.Media.Audio.IAudioEffectsPackConfiguration";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("71d7627d-70c1-536c-a8f8-6f98015a7f06")
                IAudioEffectsPackConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EffectsPackId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioEffectsPackStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StatusChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StatusChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffectsPackConfiguration = __uuidof(IAudioEffectsPackConfiguration);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Media.Audio.IAudioEffectsPackConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioEffectsPackConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioEffectsPackConfigurationStatics[] = L"Windows.Media.Audio.IAudioEffectsPackConfigurationStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("61c20413-530c-55ff-ba2b-8e68a9b56a04")
                IAudioEffectsPackConfigurationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForDeviceId(
                        HSTRING effectsPackId,
                        HSTRING deviceId,
                        ABI::Windows::Media::Audio::IAudioEffectsPackConfiguration** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsDeviceIdSupported(
                        HSTRING effectsPackId,
                        HSTRING deviceId,
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEffectsPackConfigurationStatics = __uuidof(IAudioEffectsPackConfigurationStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Media.Audio.IAudioFileInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFileInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFileInputNode[] = L"Windows.Media.Audio.IAudioFileInputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("905b67c8-6f65-4cd4-8890-4694843c276d")
                IAudioFileInputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PlaybackSpeedFactor(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PlaybackSpeedFactor(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Seek(
                        ABI::Windows::Foundation::TimeSpan position
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EndTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LoopCount(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LoopCount(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceFile(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_FileCompleted(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_FileCompleted(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioFileInputNode = __uuidof(IAudioFileInputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFileOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFileOutputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFileOutputNode[] = L"Windows.Media.Audio.IAudioFileOutputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("50e01980-5166-4093-80f8-ada00089e9cf")
                IAudioFileOutputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileEncodingProfile(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE FinalizeAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioFileOutputNode = __uuidof(IAudioFileOutputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFrameCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFrameCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFrameCompletedEventArgs[] = L"Windows.Media.Audio.IAudioFrameCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("dc7c829e-0208-4504-a5a8-f0f268920a65")
                IAudioFrameCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Frame(
                        ABI::Windows::Media::IAudioFrame** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioFrameCompletedEventArgs = __uuidof(IAudioFrameCompletedEventArgs);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFrameInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFrameInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFrameInputNode[] = L"Windows.Media.Audio.IAudioFrameInputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("01b266c7-fd96-4ff5-a3c5-d27a9bf44237")
                IAudioFrameInputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PlaybackSpeedFactor(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PlaybackSpeedFactor(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddFrame(
                        ABI::Windows::Media::IAudioFrame* frame
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DiscardQueuedFrames(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QueuedSampleCount(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AudioFrameCompleted(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AudioFrameCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_QuantumStarted(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_QuantumStarted(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioFrameInputNode = __uuidof(IAudioFrameInputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFrameOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFrameOutputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFrameOutputNode[] = L"Windows.Media.Audio.IAudioFrameOutputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("b847371b-3299-45f5-88b3-c9d12a3f1cc8")
                IAudioFrameOutputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetFrame(
                        ABI::Windows::Media::IAudioFrame** audioFrame
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioFrameOutputNode = __uuidof(IAudioFrameOutputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraph[] = L"Windows.Media.Audio.IAudioGraph";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("1ad46eed-e48c-4e14-9660-2c4f83e9cdd8")
                IAudioGraph : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFrameInputNode(
                        ABI::Windows::Media::Audio::IAudioFrameInputNode** frameInputNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFrameInputNodeWithFormat(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Media::Audio::IAudioFrameInputNode** frameInputNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDeviceInputNodeAsync(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDeviceInputNodeWithFormatAsync(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDeviceInputNodeWithFormatOnDeviceAsync(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* device,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFrameOutputNode(
                        ABI::Windows::Media::Audio::IAudioFrameOutputNode** frameOutputNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFrameOutputNodeWithFormat(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Media::Audio::IAudioFrameOutputNode** frameOutputNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDeviceOutputNodeAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFileInputNodeAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFileOutputNodeAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFileOutputNodeWithFileProfileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile* fileEncodingProfile,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSubmixNode(
                        ABI::Windows::Media::Audio::IAudioInputNode** submixNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSubmixNodeWithFormat(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Media::Audio::IAudioInputNode** submixNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ResetAllNodes(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_QuantumStarted(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_QuantumStarted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_QuantumProcessed(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_QuantumProcessed(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_UnrecoverableErrorOccurred(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_UnrecoverableErrorOccurred(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CompletedQuantumCount(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LatencyInSamples(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrimaryRenderDevice(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RenderDeviceAudioProcessing(
                        ABI::Windows::Media::AudioProcessing* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SamplesPerQuantum(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraph = __uuidof(IAudioGraph);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraph;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraph2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioGraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraph2[] = L"Windows.Media.Audio.IAudioGraph2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("4e4c3bd5-4fc1-45f6-a947-3cd38f4fd839")
                IAudioGraph2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateFrameInputNodeWithFormatAndEmitter(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Media::Audio::IAudioNodeEmitter* emitter,
                        ABI::Windows::Media::Audio::IAudioFrameInputNode** frameInputNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* device,
                        ABI::Windows::Media::Audio::IAudioNodeEmitter* emitter,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFileInputNodeWithEmitterAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        ABI::Windows::Media::Audio::IAudioNodeEmitter* emitter,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSubmixNodeWithFormatAndEmitter(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* encodingProperties,
                        ABI::Windows::Media::Audio::IAudioNodeEmitter* emitter,
                        ABI::Windows::Media::Audio::IAudioInputNode** submixNode
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBatchUpdater(
                        ABI::Windows::Foundation::IClosable** updater
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraph2 = __uuidof(IAudioGraph2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraph3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraph3[] = L"Windows.Media.Audio.IAudioGraph3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("ddcd25ae-1185-42a7-831d-6a9b0fc86820")
                IAudioGraph3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateMediaSourceAudioInputNodeAsync(
                        ABI::Windows::Media::Core::IMediaSource2* mediaSource,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMediaSourceAudioInputNodeWithEmitterAsync(
                        ABI::Windows::Media::Core::IMediaSource2* mediaSource,
                        ABI::Windows::Media::Audio::IAudioNodeEmitter* emitter,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraph3 = __uuidof(IAudioGraph3);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphConnection[] = L"Windows.Media.Audio.IAudioGraphConnection";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("763070ed-d04e-4fac-b233-600b42edd469")
                IAudioGraphConnection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Destination(
                        ABI::Windows::Media::Audio::IAudioNode** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Gain(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gain(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraphConnection = __uuidof(IAudioGraphConnection);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphSettings[] = L"Windows.Media.Audio.IAudioGraphSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("1d59647f-e6fe-4628-84f8-9d8bdba25785")
                IAudioGraphSettings : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PrimaryRenderDevice(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PrimaryRenderDevice(
                        ABI::Windows::Devices::Enumeration::IDeviceInformation* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_QuantumSizeSelectionMode(
                        ABI::Windows::Media::Audio::QuantumSizeSelectionMode* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_QuantumSizeSelectionMode(
                        ABI::Windows::Media::Audio::QuantumSizeSelectionMode value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredSamplesPerQuantum(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredSamplesPerQuantum(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AudioRenderCategory(
                        ABI::Windows::Media::Render::AudioRenderCategory* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AudioRenderCategory(
                        ABI::Windows::Media::Render::AudioRenderCategory value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DesiredRenderDeviceAudioProcessing(
                        ABI::Windows::Media::AudioProcessing* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DesiredRenderDeviceAudioProcessing(
                        ABI::Windows::Media::AudioProcessing value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraphSettings = __uuidof(IAudioGraphSettings);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphSettings2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphSettings2[] = L"Windows.Media.Audio.IAudioGraphSettings2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("72919787-4dab-46e3-b4c9-d8e1a2636062")
                IAudioGraphSettings2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_MaxPlaybackSpeedFactor(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxPlaybackSpeedFactor(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraphSettings2 = __uuidof(IAudioGraphSettings2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphSettingsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphSettingsFactory[] = L"Windows.Media.Audio.IAudioGraphSettingsFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("a5d91cc6-c2eb-4a61-a214-1d66d75f83da")
                IAudioGraphSettingsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::Render::AudioRenderCategory audioRenderCategory,
                        ABI::Windows::Media::Audio::IAudioGraphSettings** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraphSettingsFactory = __uuidof(IAudioGraphSettingsFactory);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphStatics[] = L"Windows.Media.Audio.IAudioGraphStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("76ec3132-e159-4ab7-a82a-17beb4b31e94")
                IAudioGraphStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        ABI::Windows::Media::Audio::IAudioGraphSettings* settings,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraphStatics = __uuidof(IAudioGraphStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphUnrecoverableErrorOccurredEventArgs[] = L"Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("c3d9cbe0-3ff6-4fb3-b262-50d435c55423")
                IAudioGraphUnrecoverableErrorOccurredEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Error(
                        ABI::Windows::Media::Audio::AudioGraphUnrecoverableError* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioGraphUnrecoverableErrorOccurredEventArgs = __uuidof(IAudioGraphUnrecoverableErrorOccurredEventArgs);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioInputNode[] = L"Windows.Media.Audio.IAudioInputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("d148005c-8428-4784-b7fd-a99d468c5d20")
                IAudioInputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_OutgoingConnections(
                        __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddOutgoingConnection(
                        ABI::Windows::Media::Audio::IAudioNode* destination
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AddOutgoingConnectionWithGain(
                        ABI::Windows::Media::Audio::IAudioNode* destination,
                        DOUBLE gain
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveOutgoingConnection(
                        ABI::Windows::Media::Audio::IAudioNode* destination
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioInputNode = __uuidof(IAudioInputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioInputNode2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioInputNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioInputNode2[] = L"Windows.Media.Audio.IAudioInputNode2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("905156b7-ca68-4c6d-a8bc-e3ee17fe3fd2")
                IAudioInputNode2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Emitter(
                        ABI::Windows::Media::Audio::IAudioNodeEmitter** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioInputNode2 = __uuidof(IAudioInputNode2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNode[] = L"Windows.Media.Audio.IAudioNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("15389d7f-dbd8-4819-bf03-668e9357cd6d")
                IAudioNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_EffectDefinitions(
                        __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OutgoingGain(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OutgoingGain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EncodingProperties(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConsumeInput(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ConsumeInput(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Stop(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Reset(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE DisableEffectsByDefinition(
                        ABI::Windows::Media::Effects::IAudioEffectDefinition* definition
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE EnableEffectsByDefinition(
                        ABI::Windows::Media::Effects::IAudioEffectDefinition* definition
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNode = __uuidof(IAudioNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitter[] = L"Windows.Media.Audio.IAudioNodeEmitter";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("3676971d-880a-47b8-adf7-1323a9d965be")
                IAudioNodeEmitter : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Position(
                        ABI::Windows::Foundation::Numerics::Vector3 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Direction(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Direction(
                        ABI::Windows::Foundation::Numerics::Vector3 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Shape(
                        ABI::Windows::Media::Audio::IAudioNodeEmitterShape** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DecayModel(
                        ABI::Windows::Media::Audio::IAudioNodeEmitterDecayModel** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Gain(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DistanceScale(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DistanceScale(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DopplerScale(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DopplerScale(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DopplerVelocity(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DopplerVelocity(
                        ABI::Windows::Foundation::Numerics::Vector3 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDopplerDisabled(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitter = __uuidof(IAudioNodeEmitter);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitter2[] = L"Windows.Media.Audio.IAudioNodeEmitter2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("4ab6eecb-ec29-47f8-818c-b6b660a5aeb1")
                IAudioNodeEmitter2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SpatialAudioModel(
                        ABI::Windows::Media::Audio::SpatialAudioModel* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SpatialAudioModel(
                        ABI::Windows::Media::Audio::SpatialAudioModel value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitter2 = __uuidof(IAudioNodeEmitter2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterConeProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterConeProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterConeProperties[] = L"Windows.Media.Audio.IAudioNodeEmitterConeProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("e99b2cee-02ca-4375-9326-0c6ae4bcdfb5")
                IAudioNodeEmitterConeProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InnerAngle(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OuterAngle(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OuterAngleGain(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterConeProperties = __uuidof(IAudioNodeEmitterConeProperties);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterDecayModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterDecayModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterDecayModel[] = L"Windows.Media.Audio.IAudioNodeEmitterDecayModel";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("1d1d5af7-0d53-4fa9-bd84-d5816a86f3ff")
                IAudioNodeEmitterDecayModel : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Media::Audio::AudioNodeEmitterDecayKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MinGain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MaxGain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_NaturalProperties(
                        ABI::Windows::Media::Audio::IAudioNodeEmitterNaturalDecayModelProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterDecayModel = __uuidof(IAudioNodeEmitterDecayModel);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterDecayModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterDecayModelStatics[] = L"Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("c7787ca8-f178-462f-bc81-8dd5cbe5dae8")
                IAudioNodeEmitterDecayModelStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateNatural(
                        DOUBLE minGain,
                        DOUBLE maxGain,
                        DOUBLE unityGainDistance,
                        DOUBLE cutoffDistance,
                        ABI::Windows::Media::Audio::IAudioNodeEmitterDecayModel** decayModel
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateCustom(
                        DOUBLE minGain,
                        DOUBLE maxGain,
                        ABI::Windows::Media::Audio::IAudioNodeEmitterDecayModel** decayModel
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterDecayModelStatics = __uuidof(IAudioNodeEmitterDecayModelStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterFactory[] = L"Windows.Media.Audio.IAudioNodeEmitterFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("fdc8489a-6ad6-4ce4-b7f7-a99370df7ee9")
                IAudioNodeEmitterFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAudioNodeEmitter(
                        ABI::Windows::Media::Audio::IAudioNodeEmitterShape* shape,
                        ABI::Windows::Media::Audio::IAudioNodeEmitterDecayModel* decayModel,
                        ABI::Windows::Media::Audio::AudioNodeEmitterSettings settings,
                        ABI::Windows::Media::Audio::IAudioNodeEmitter** emitter
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterFactory = __uuidof(IAudioNodeEmitterFactory);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterNaturalDecayModelProperties[] = L"Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("48934bcf-cf2c-4efc-9331-75bd22df1f0c")
                IAudioNodeEmitterNaturalDecayModelProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UnityGainDistance(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CutoffDistance(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterNaturalDecayModelProperties = __uuidof(IAudioNodeEmitterNaturalDecayModelProperties);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterShape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterShape[] = L"Windows.Media.Audio.IAudioNodeEmitterShape";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("ea0311c5-e73d-44bc-859c-45553bbc4828")
                IAudioNodeEmitterShape : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Kind(
                        ABI::Windows::Media::Audio::AudioNodeEmitterShapeKind* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ConeProperties(
                        ABI::Windows::Media::Audio::IAudioNodeEmitterConeProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterShape = __uuidof(IAudioNodeEmitterShape);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterShapeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterShape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterShapeStatics[] = L"Windows.Media.Audio.IAudioNodeEmitterShapeStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("57bb2771-ffa5-4b86-a779-e264aeb9145f")
                IAudioNodeEmitterShapeStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateCone(
                        DOUBLE innerAngle,
                        DOUBLE outerAngle,
                        DOUBLE outerAngleGain,
                        ABI::Windows::Media::Audio::IAudioNodeEmitterShape** shape
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateOmnidirectional(
                        ABI::Windows::Media::Audio::IAudioNodeEmitterShape** shape
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeEmitterShapeStatics = __uuidof(IAudioNodeEmitterShapeStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeListener[] = L"Windows.Media.Audio.IAudioNodeListener";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("d9722e16-0c0a-41da-b755-6c77835fb1eb")
                IAudioNodeListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Position(
                        ABI::Windows::Foundation::Numerics::Vector3 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Orientation(
                        ABI::Windows::Foundation::Numerics::Quaternion* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Orientation(
                        ABI::Windows::Foundation::Numerics::Quaternion value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SpeedOfSound(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SpeedOfSound(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DopplerVelocity(
                        ABI::Windows::Foundation::Numerics::Vector3* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DopplerVelocity(
                        ABI::Windows::Foundation::Numerics::Vector3 value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeListener = __uuidof(IAudioNodeListener);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeWithListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeWithListener[] = L"Windows.Media.Audio.IAudioNodeWithListener";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("0e0f907c-79ff-4544-9eeb-01257b15105a")
                IAudioNodeWithListener : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Listener(
                        ABI::Windows::Media::Audio::IAudioNodeListener* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Listener(
                        ABI::Windows::Media::Audio::IAudioNodeListener** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioNodeWithListener = __uuidof(IAudioNodeWithListener);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioPlaybackConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioPlaybackConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioPlaybackConnection[] = L"Windows.Media.Audio.IAudioPlaybackConnection";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("1a4c1dea-cafc-50e7-8718-ea3f81cbfa51")
                IAudioPlaybackConnection : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Media::Audio::AudioPlaybackConnectionState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Open(
                        ABI::Windows::Media::Audio::IAudioPlaybackConnectionOpenResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE OpenAsync(
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_StateChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_StateChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioPlaybackConnection = __uuidof(IAudioPlaybackConnection);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Audio.IAudioPlaybackConnectionOpenResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioPlaybackConnectionOpenResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioPlaybackConnectionOpenResult[] = L"Windows.Media.Audio.IAudioPlaybackConnectionOpenResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("4e656aef-39f9-5fc9-a519-a5bbfd9fe921")
                IAudioPlaybackConnectionOpenResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioPlaybackConnectionOpenResultStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioPlaybackConnectionOpenResult = __uuidof(IAudioPlaybackConnectionOpenResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Audio.IAudioPlaybackConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioPlaybackConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioPlaybackConnectionStatics[] = L"Windows.Media.Audio.IAudioPlaybackConnectionStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("e60963a2-69e6-5ffc-9e13-824a85213daf")
                IAudioPlaybackConnectionStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE TryCreateFromId(
                        HSTRING id,
                        ABI::Windows::Media::Audio::IAudioPlaybackConnection** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioPlaybackConnectionStatics = __uuidof(IAudioPlaybackConnectionStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Audio.IAudioStateMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioStateMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioStateMonitor[] = L"Windows.Media.Audio.IAudioStateMonitor";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("1d13d136-0199-4cdc-b84e-e72c2b581ece")
                IAudioStateMonitor : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_SoundLevelChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_SoundLevelChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SoundLevel(
                        ABI::Windows::Media::SoundLevel* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioStateMonitor = __uuidof(IAudioStateMonitor);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IAudioStateMonitorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioStateMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioStateMonitorStatics[] = L"Windows.Media.Audio.IAudioStateMonitorStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("6374ea4c-1b3b-4001-94d9-dd225330fa40")
                IAudioStateMonitorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateForRenderMonitoring(
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForRenderMonitoringWithCategory(
                        ABI::Windows::Media::Render::AudioRenderCategory category,
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForRenderMonitoringWithCategoryAndDeviceRole(
                        ABI::Windows::Media::Render::AudioRenderCategory category,
                        ABI::Windows::Media::Devices::AudioDeviceRole role,
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForRenderMonitoringWithCategoryAndDeviceId(
                        ABI::Windows::Media::Render::AudioRenderCategory category,
                        HSTRING deviceId,
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForCaptureMonitoring(
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForCaptureMonitoringWithCategory(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForCaptureMonitoringWithCategoryAndDeviceRole(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        ABI::Windows::Media::Devices::AudioDeviceRole role,
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateForCaptureMonitoringWithCategoryAndDeviceId(
                        ABI::Windows::Media::Capture::MediaCategory category,
                        HSTRING deviceId,
                        ABI::Windows::Media::Audio::IAudioStateMonitor** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioStateMonitorStatics = __uuidof(IAudioStateMonitorStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceInputNodeResult[] = L"Windows.Media.Audio.ICreateAudioDeviceInputNodeResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("16eec7a8-1ca7-40ef-91a4-d346e0aa1bba")
                ICreateAudioDeviceInputNodeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioDeviceNodeCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceInputNode(
                        ABI::Windows::Media::Audio::IAudioDeviceInputNode** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioDeviceInputNodeResult = __uuidof(ICreateAudioDeviceInputNodeResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceInputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("921c69ce-3f35-41c7-9622-79f608baedc2")
                ICreateAudioDeviceInputNodeResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioDeviceInputNodeResult2 = __uuidof(ICreateAudioDeviceInputNodeResult2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceOutputNodeResult[] = L"Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("f7776d27-1d9a-47f7-9cd4-2859cc1b7bff")
                ICreateAudioDeviceOutputNodeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioDeviceNodeCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceOutputNode(
                        ABI::Windows::Media::Audio::IAudioDeviceOutputNode** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioDeviceOutputNodeResult = __uuidof(ICreateAudioDeviceOutputNodeResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceOutputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("4864269f-bdce-4ab1-bd38-fbae93aedaca")
                ICreateAudioDeviceOutputNodeResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioDeviceOutputNodeResult2 = __uuidof(ICreateAudioDeviceOutputNodeResult2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileInputNodeResult[] = L"Windows.Media.Audio.ICreateAudioFileInputNodeResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("ce83d61c-e297-4c50-9ce7-1c7a69d6bd09")
                ICreateAudioFileInputNodeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioFileNodeCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileInputNode(
                        ABI::Windows::Media::Audio::IAudioFileInputNode** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioFileInputNodeResult = __uuidof(ICreateAudioFileInputNodeResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileInputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileInputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioFileInputNodeResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("f9082020-3d80-4fe0-81c1-768fea7ca7e0")
                ICreateAudioFileInputNodeResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioFileInputNodeResult2 = __uuidof(ICreateAudioFileInputNodeResult2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileOutputNodeResult[] = L"Windows.Media.Audio.ICreateAudioFileOutputNodeResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("47d6ba7b-e909-453f-866e-5540cda734ff")
                ICreateAudioFileOutputNodeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioFileNodeCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileOutputNode(
                        ABI::Windows::Media::Audio::IAudioFileOutputNode** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioFileOutputNodeResult = __uuidof(ICreateAudioFileOutputNodeResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileOutputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileOutputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioFileOutputNodeResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("9f01b50d-3318-47b3-a60a-1b492be7fc0d")
                ICreateAudioFileOutputNodeResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioFileOutputNodeResult2 = __uuidof(ICreateAudioFileOutputNodeResult2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioGraphResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioGraphResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioGraphResult[] = L"Windows.Media.Audio.ICreateAudioGraphResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("5453ef7e-7bde-4b76-bb5d-48f79cfc8c0b")
                ICreateAudioGraphResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::AudioGraphCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Graph(
                        ABI::Windows::Media::Audio::IAudioGraph** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioGraphResult = __uuidof(ICreateAudioGraphResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioGraphResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioGraphResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioGraphResult2[] = L"Windows.Media.Audio.ICreateAudioGraphResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("6d738dfc-88c6-4fcb-a534-85cedd4050a1")
                ICreateAudioGraphResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateAudioGraphResult2 = __uuidof(ICreateAudioGraphResult2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateMediaSourceAudioInputNodeResult[] = L"Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("46a658a3-53c0-4d59-9e51-cc1d1044a4c4")
                ICreateMediaSourceAudioInputNodeResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::MediaSourceAudioInputNodeCreationStatus* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Node(
                        ABI::Windows::Media::Audio::IMediaSourceAudioInputNode** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateMediaSourceAudioInputNodeResult = __uuidof(ICreateMediaSourceAudioInputNodeResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateMediaSourceAudioInputNodeResult2[] = L"Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("63514ce8-6a1a-49e3-97ec-28fd5be114e5")
                ICreateMediaSourceAudioInputNodeResult2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ICreateMediaSourceAudioInputNodeResult2 = __uuidof(ICreateMediaSourceAudioInputNodeResult2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.IEchoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EchoEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEchoEffectDefinition[] = L"Windows.Media.Audio.IEchoEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("0e4d3faa-36b8-4c91-b9da-11f44a8a6610")
                IEchoEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_WetDryMix(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WetDryMix(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Feedback(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Feedback(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Delay(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Delay(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEchoEffectDefinition = __uuidof(IEchoEffectDefinition);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEchoEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EchoEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEchoEffectDefinitionFactory[] = L"Windows.Media.Audio.IEchoEffectDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("0d4e2257-aaf2-4e86-a54c-fb79db8f6c12")
                IEchoEffectDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::Audio::IAudioGraph* audioGraph,
                        ABI::Windows::Media::Audio::IEchoEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEchoEffectDefinitionFactory = __uuidof(IEchoEffectDefinitionFactory);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEqualizerBand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EqualizerBand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEqualizerBand[] = L"Windows.Media.Audio.IEqualizerBand";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("c00a5a6a-262d-4b85-9bb7-43280b62ed0c")
                IEqualizerBand : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Bandwidth(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Bandwidth(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FrequencyCenter(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FrequencyCenter(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Gain(
                        DOUBLE value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEqualizerBand = __uuidof(IEqualizerBand);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEqualizerEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EqualizerEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEqualizerEffectDefinition[] = L"Windows.Media.Audio.IEqualizerEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("023f6f1f-83fe-449a-a822-c696442d16b0")
                IEqualizerEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Bands(
                        __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEqualizerEffectDefinition = __uuidof(IEqualizerEffectDefinition);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEqualizerEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EqualizerEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEqualizerEffectDefinitionFactory[] = L"Windows.Media.Audio.IEqualizerEffectDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("d2876fc4-d410-4eb5-9e69-c9aa1277eaf0")
                IEqualizerEffectDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::Audio::IAudioGraph* audioGraph,
                        ABI::Windows::Media::Audio::IEqualizerEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IEqualizerEffectDefinitionFactory = __uuidof(IEqualizerEffectDefinitionFactory);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IFrameInputNodeQuantumStartedEventArgs[] = L"Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("3d9bd498-a306-4f06-bd9f-e9efc8226304")
                IFrameInputNodeQuantumStartedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_RequiredSamples(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IFrameInputNodeQuantumStartedEventArgs = __uuidof(IFrameInputNodeQuantumStartedEventArgs);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ILimiterEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.LimiterEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ILimiterEffectDefinition[] = L"Windows.Media.Audio.ILimiterEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("6b755d19-2603-47ba-bdeb-39055e3486dc")
                ILimiterEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Release(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Release(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Loudness(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Loudness(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILimiterEffectDefinition = __uuidof(ILimiterEffectDefinition);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ILimiterEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.LimiterEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ILimiterEffectDefinitionFactory[] = L"Windows.Media.Audio.ILimiterEffectDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("ecbae6f1-61ff-45ef-b8f5-48659a57c72d")
                ILimiterEffectDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::Audio::IAudioGraph* audioGraph,
                        ABI::Windows::Media::Audio::ILimiterEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ILimiterEffectDefinitionFactory = __uuidof(ILimiterEffectDefinitionFactory);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IMediaSourceAudioInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.MediaSourceAudioInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioInputNode2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IMediaSourceAudioInputNode[] = L"Windows.Media.Audio.IMediaSourceAudioInputNode";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("99d8983b-a88a-4041-8e4f-ddbac0c91fd3")
                IMediaSourceAudioInputNode : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_PlaybackSpeedFactor(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PlaybackSpeedFactor(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Position(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Seek(
                        ABI::Windows::Foundation::TimeSpan position
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_StartTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EndTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EndTime(
                        __FIReference_1_Windows__CFoundation__CTimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LoopCount(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LoopCount(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Duration(
                        ABI::Windows::Foundation::TimeSpan* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MediaSource(
                        ABI::Windows::Media::Core::IMediaSource2** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_MediaSourceCompleted(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_MediaSourceCompleted(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaSourceAudioInputNode = __uuidof(IMediaSourceAudioInputNode);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IReverbEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.ReverbEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IReverbEffectDefinition[] = L"Windows.Media.Audio.IReverbEffectDefinition";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("4606aa89-f563-4d0a-8f6e-f0cddff35d84")
                IReverbEffectDefinition : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_WetDryMix(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_WetDryMix(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReflectionsDelay(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReflectionsDelay(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReverbDelay(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReverbDelay(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RearDelay(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RearDelay(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PositionLeft(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PositionLeft(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PositionRight(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PositionRight(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PositionMatrixLeft(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PositionMatrixLeft(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PositionMatrixRight(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PositionMatrixRight(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_EarlyDiffusion(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EarlyDiffusion(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LateDiffusion(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LateDiffusion(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LowEQGain(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LowEQGain(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_LowEQCutoff(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LowEQCutoff(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HighEQGain(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HighEQGain(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_HighEQCutoff(
                        BYTE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HighEQCutoff(
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoomFilterFreq(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoomFilterFreq(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoomFilterMain(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoomFilterMain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoomFilterHF(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoomFilterHF(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReflectionsGain(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReflectionsGain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ReverbGain(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ReverbGain(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DecayTime(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DecayTime(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Density(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Density(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RoomSize(
                        DOUBLE value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RoomSize(
                        DOUBLE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_DisableLateField(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DisableLateField(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IReverbEffectDefinition = __uuidof(IReverbEffectDefinition);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IReverbEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.ReverbEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IReverbEffectDefinitionFactory[] = L"Windows.Media.Audio.IReverbEffectDefinitionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("a7d5cbfe-100b-4ff0-9da6-dc4e05a759f0")
                IReverbEffectDefinitionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Create(
                        ABI::Windows::Media::Audio::IAudioGraph* audioGraph,
                        ABI::Windows::Media::Audio::IReverbEffectDefinition** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IReverbEffectDefinitionFactory = __uuidof(IReverbEffectDefinitionFactory);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SetDefaultSpatialAudioFormatResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISetDefaultSpatialAudioFormatResult[] = L"Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("1c2aa511-1400-5e70-9ea9-ae151241e8ea")
                ISetDefaultSpatialAudioFormatResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Media::Audio::SetDefaultSpatialAudioFormatStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISetDefaultSpatialAudioFormatResult = __uuidof(ISetDefaultSpatialAudioFormatResult);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioDeviceConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioDeviceConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioDeviceConfiguration[] = L"Windows.Media.Audio.ISpatialAudioDeviceConfiguration";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("ee830034-61cf-5749-9da4-10f0fe028199")
                ISpatialAudioDeviceConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DeviceId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSpatialAudioSupported(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsSpatialAudioFormatSupported(
                        HSTRING subtype,
                        boolean* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActiveSpatialAudioFormat(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DefaultSpatialAudioFormat(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetDefaultSpatialAudioFormatAsync(
                        HSTRING subtype,
                        __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ConfigurationChanged(
                        __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ConfigurationChanged(
                        EventRegistrationToken token
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAudioDeviceConfiguration = __uuidof(ISpatialAudioDeviceConfiguration);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioDeviceConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioDeviceConfigurationStatics[] = L"Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("3ec37f7b-936d-4e04-9728-2827d9f758c4")
                ISpatialAudioDeviceConfigurationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetForDeviceId(
                        HSTRING deviceId,
                        ABI::Windows::Media::Audio::ISpatialAudioDeviceConfiguration** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAudioDeviceConfigurationStatics = __uuidof(ISpatialAudioDeviceConfigurationStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatConfiguration[] = L"Windows.Media.Audio.ISpatialAudioFormatConfiguration";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("32df09a8-50f0-5395-9923-7d44ca71ed6d")
                ISpatialAudioFormatConfiguration : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE ReportLicenseChangedAsync(
                        HSTRING subtype,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ReportConfigurationChangedAsync(
                        HSTRING subtype,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MixedRealityExclusiveModePolicy(
                        ABI::Windows::Media::Audio::MixedRealitySpatialAudioFormatPolicy* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MixedRealityExclusiveModePolicy(
                        ABI::Windows::Media::Audio::MixedRealitySpatialAudioFormatPolicy value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAudioFormatConfiguration = __uuidof(ISpatialAudioFormatConfiguration);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatConfigurationStatics[] = L"Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("2b5fef71-67c9-4e5f-a35b-41680711f8c7")
                ISpatialAudioFormatConfigurationStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Media::Audio::ISpatialAudioFormatConfiguration** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAudioFormatConfigurationStatics = __uuidof(ISpatialAudioFormatConfigurationStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatSubtype
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatSubtypeStatics[] = L"Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("b3de8a47-83ee-4266-a945-bedf507afeed")
                ISpatialAudioFormatSubtypeStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WindowsSonic(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DolbyAtmosForHeadphones(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DolbyAtmosForHomeTheater(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DolbyAtmosForSpeakers(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DTSHeadphoneX(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DTSXUltra(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAudioFormatSubtypeStatics = __uuidof(ISpatialAudioFormatSubtypeStatics);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatSubtype
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatSubtypeStatics2[] = L"Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Audio {
                MIDL_INTERFACE("4565e6cb-d95b-5621-b6af-0e8849c57c80")
                ISpatialAudioFormatSubtypeStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DTSXForHomeTheater(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ISpatialAudioFormatSubtypeStatics2 = __uuidof(ISpatialAudioFormatSubtypeStatics2);
            } /* Audio */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Audio.AudioDeviceInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioDeviceInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioDeviceInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioDeviceInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioDeviceInputNode[] = L"Windows.Media.Audio.AudioDeviceInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioDeviceOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioDeviceOutputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioNodeWithListener
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioDeviceOutputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioDeviceOutputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioDeviceOutputNode[] = L"Windows.Media.Audio.AudioDeviceOutputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioEffectsPackConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioEffectsPackConfigurationStatics interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioEffectsPackConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioEffectsPackConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioEffectsPackConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioEffectsPackConfiguration[] = L"Windows.Media.Audio.AudioEffectsPackConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Media.Audio.AudioFileInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFileInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFileInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFileInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFileInputNode[] = L"Windows.Media.Audio.AudioFileInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFileOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFileOutputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFileOutputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFileOutputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFileOutputNode[] = L"Windows.Media.Audio.AudioFileOutputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFrameCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFrameCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFrameCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFrameCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFrameCompletedEventArgs[] = L"Windows.Media.Audio.AudioFrameCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFrameInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFrameInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFrameInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFrameInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFrameInputNode[] = L"Windows.Media.Audio.AudioFrameInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFrameOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFrameOutputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFrameOutputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFrameOutputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFrameOutputNode[] = L"Windows.Media.Audio.AudioFrameOutputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioGraphStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraph ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioGraph2
 *    Windows.Media.Audio.IAudioGraph3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraph_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraph_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraph[] = L"Windows.Media.Audio.AudioGraph";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraphBatchUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IClosable ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphBatchUpdater_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphBatchUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphBatchUpdater[] = L"Windows.Media.Audio.AudioGraphBatchUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioGraphConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraphConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphConnection[] = L"Windows.Media.Audio.AudioGraphConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraphSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IAudioGraphSettingsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraphSettings ** Default Interface **
 *    Windows.Media.Audio.IAudioGraphSettings2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphSettings[] = L"Windows.Media.Audio.AudioGraphSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphUnrecoverableErrorOccurredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphUnrecoverableErrorOccurredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphUnrecoverableErrorOccurredEventArgs[] = L"Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Media.Audio.IAudioNodeEmitterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitter ** Default Interface **
 *    Windows.Media.Audio.IAudioNodeEmitter2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitter_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitter[] = L"Windows.Media.Audio.AudioNodeEmitter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterConeProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterConeProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterConeProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterConeProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterConeProperties[] = L"Windows.Media.Audio.AudioNodeEmitterConeProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterDecayModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterDecayModel ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterDecayModel_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterDecayModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterDecayModel[] = L"Windows.Media.Audio.AudioNodeEmitterDecayModel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterNaturalDecayModelProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterNaturalDecayModelProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterNaturalDecayModelProperties[] = L"Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioNodeEmitterShapeStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterShape ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterShape_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterShape_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterShape[] = L"Windows.Media.Audio.AudioNodeEmitterShape";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeListener_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeListener[] = L"Windows.Media.Audio.AudioNodeListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioPlaybackConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioPlaybackConnectionStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioPlaybackConnection ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioPlaybackConnection[] = L"Windows.Media.Audio.AudioPlaybackConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Media.Audio.AudioPlaybackConnectionOpenResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioPlaybackConnectionOpenResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnectionOpenResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnectionOpenResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioPlaybackConnectionOpenResult[] = L"Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Media.Audio.AudioStateMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioStateMonitorStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioStateMonitor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioStateMonitor_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioStateMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioStateMonitor[] = L"Windows.Media.Audio.AudioStateMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Audio.AudioSubmixNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioSubmixNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioSubmixNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioSubmixNode[] = L"Windows.Media.Audio.AudioSubmixNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioDeviceInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioDeviceInputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceInputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceInputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioDeviceInputNodeResult[] = L"Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioDeviceOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceOutputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceOutputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioDeviceOutputNodeResult[] = L"Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioFileInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioFileInputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioFileInputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileInputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileInputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioFileInputNodeResult[] = L"Windows.Media.Audio.CreateAudioFileInputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioFileOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioFileOutputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioFileOutputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileOutputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileOutputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioFileOutputNodeResult[] = L"Windows.Media.Audio.CreateAudioFileOutputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioGraphResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioGraphResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioGraphResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioGraphResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioGraphResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioGraphResult[] = L"Windows.Media.Audio.CreateAudioGraphResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateMediaSourceAudioInputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateMediaSourceAudioInputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateMediaSourceAudioInputNodeResult[] = L"Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Audio.EchoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IEchoEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IEchoEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_EchoEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_EchoEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_EchoEffectDefinition[] = L"Windows.Media.Audio.EchoEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.EqualizerBand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IEqualizerBand ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_EqualizerBand_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_EqualizerBand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_EqualizerBand[] = L"Windows.Media.Audio.EqualizerBand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.EqualizerEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IEqualizerEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IEqualizerEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_EqualizerEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_EqualizerEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_EqualizerEffectDefinition[] = L"Windows.Media.Audio.EqualizerEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_FrameInputNodeQuantumStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_FrameInputNodeQuantumStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_FrameInputNodeQuantumStartedEventArgs[] = L"Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.LimiterEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.ILimiterEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ILimiterEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_LimiterEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_LimiterEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_LimiterEffectDefinition[] = L"Windows.Media.Audio.LimiterEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.MediaSourceAudioInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IMediaSourceAudioInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode2
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Audio_MediaSourceAudioInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_MediaSourceAudioInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_MediaSourceAudioInputNode[] = L"Windows.Media.Audio.MediaSourceAudioInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Audio.ReverbEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IReverbEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IReverbEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_ReverbEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_ReverbEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_ReverbEffectDefinition[] = L"Windows.Media.Audio.ReverbEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.SetDefaultSpatialAudioFormatResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SetDefaultSpatialAudioFormatResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SetDefaultSpatialAudioFormatResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SetDefaultSpatialAudioFormatResult[] = L"Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Audio.SpatialAudioDeviceConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ISpatialAudioDeviceConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SpatialAudioDeviceConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SpatialAudioDeviceConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SpatialAudioDeviceConfiguration[] = L"Windows.Media.Audio.SpatialAudioDeviceConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Audio.SpatialAudioFormatConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ISpatialAudioFormatConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SpatialAudioFormatConfiguration[] = L"Windows.Media.Audio.SpatialAudioFormatConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Audio.SpatialAudioFormatSubtype
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2 interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatSubtype_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatSubtype_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SpatialAudioFormatSubtype[] = L"Windows.Media.Audio.SpatialAudioFormatSubtype";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2 __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3 __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2 __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2 __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNode __x_ABI_CWindows_CMedia_CAudio_CIAudioNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2 __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2 __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2 __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2 __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2 __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2 __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2 __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2 __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2;

#endif // ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateAudioGraphResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReasonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        enum __x_ABI_CWindows_CMedia_CTranscoding_CTranscodeFailureReason* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReasonVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReasonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReasonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* This,
        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReasonVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReasonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection;

typedef struct __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl;

interface __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection;

typedef struct __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        __FIIterator_1_Windows__CMedia__CAudio__CAudioGraphConnection** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl;

interface __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand;

typedef struct __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBandVtbl;

interface __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand;

typedef struct __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        __FIIterator_1_Windows__CMedia__CAudio__CEqualizerBand** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBandVtbl;

interface __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition;

#endif // ____x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __FIIterator_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection;

typedef struct __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl;

interface __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand;

typedef struct __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBandVtbl;

interface __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition;

typedef struct __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __FIVectorView_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl;

interface __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIReference_1_int_INTERFACE_DEFINED__)
#define ____FIReference_1_int_INTERFACE_DEFINED__

typedef interface __FIReference_1_int __FIReference_1_int;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_int;

typedef struct __FIReference_1_intVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_int* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_int* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_int* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_int* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_int* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_int* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_int* This,
        INT32* result);

    END_INTERFACE
} __FIReference_1_intVtbl;

interface __FIReference_1_int
{
    CONST_VTBL struct __FIReference_1_intVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_int_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_int_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_int_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_int_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_int_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_int_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_int_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_int_INTERFACE_DEFINED__

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* sender,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* sender,
        __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* sender,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable* This,
        __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

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

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion;

typedef struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 __x_ABI_CWindows_CFoundation_CNumerics_CVector3;

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAudioFrame __x_ABI_CWindows_CMedia_CIAudioFrame;

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CAudioProcessing __x_ABI_CWindows_CMedia_CAudioProcessing;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory __x_ABI_CWindows_CMedia_CCapture_CMediaCategory;

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaSource2 __x_ABI_CWindows_CMedia_CCore_CIMediaSource2;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource2_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole;

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory;

typedef enum __x_ABI_CWindows_CMedia_CSoundLevel __x_ABI_CWindows_CMedia_CSoundLevel;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioDeviceNodeCreationStatus __x_ABI_CWindows_CMedia_CAudio_CAudioDeviceNodeCreationStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioEffectsPackStatus __x_ABI_CWindows_CMedia_CAudio_CAudioEffectsPackStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioFileNodeCreationStatus __x_ABI_CWindows_CMedia_CAudio_CAudioFileNodeCreationStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioGraphCreationStatus __x_ABI_CWindows_CMedia_CAudio_CAudioGraphCreationStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioGraphUnrecoverableError __x_ABI_CWindows_CMedia_CAudio_CAudioGraphUnrecoverableError;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterDecayKind __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterDecayKind;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterSettings __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterSettings;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterShapeKind __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterShapeKind;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionOpenResultStatus __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionOpenResultStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionState __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionState;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CMediaSourceAudioInputNodeCreationStatus __x_ABI_CWindows_CMedia_CAudio_CMediaSourceAudioInputNodeCreationStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CMixedRealitySpatialAudioFormatPolicy __x_ABI_CWindows_CMedia_CAudio_CMixedRealitySpatialAudioFormatPolicy;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CQuantumSizeSelectionMode __x_ABI_CWindows_CMedia_CAudio_CQuantumSizeSelectionMode;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CSetDefaultSpatialAudioFormatStatus __x_ABI_CWindows_CMedia_CAudio_CSetDefaultSpatialAudioFormatStatus;

typedef enum __x_ABI_CWindows_CMedia_CAudio_CSpatialAudioModel __x_ABI_CWindows_CMedia_CAudio_CSpatialAudioModel;

/*
 *
 * Struct Windows.Media.Audio.AudioDeviceNodeCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioDeviceNodeCreationStatus
{
    AudioDeviceNodeCreationStatus_Success = 0,
    AudioDeviceNodeCreationStatus_DeviceNotAvailable = 1,
    AudioDeviceNodeCreationStatus_FormatNotSupported = 2,
    AudioDeviceNodeCreationStatus_UnknownFailure = 3,
    AudioDeviceNodeCreationStatus_AccessDenied = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioEffectsPackStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioEffectsPackStatus
{
    AudioEffectsPackStatus_NotEnabled = 0,
    AudioEffectsPackStatus_Enabled = 1,
    AudioEffectsPackStatus_NotSupported = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Struct Windows.Media.Audio.AudioFileNodeCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioFileNodeCreationStatus
{
    AudioFileNodeCreationStatus_Success = 0,
    AudioFileNodeCreationStatus_FileNotFound = 1,
    AudioFileNodeCreationStatus_InvalidFileType = 2,
    AudioFileNodeCreationStatus_FormatNotSupported = 3,
    AudioFileNodeCreationStatus_UnknownFailure = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioGraphCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioGraphCreationStatus
{
    AudioGraphCreationStatus_Success = 0,
    AudioGraphCreationStatus_DeviceNotAvailable = 1,
    AudioGraphCreationStatus_FormatNotSupported = 2,
    AudioGraphCreationStatus_UnknownFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioGraphUnrecoverableError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioGraphUnrecoverableError
{
    AudioGraphUnrecoverableError_None = 0,
    AudioGraphUnrecoverableError_AudioDeviceLost = 1,
    AudioGraphUnrecoverableError_AudioSessionDisconnected = 2,
    AudioGraphUnrecoverableError_UnknownFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.AudioNodeEmitterDecayKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterDecayKind
{
    AudioNodeEmitterDecayKind_Natural = 0,
    AudioNodeEmitterDecayKind_Custom = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Audio.AudioNodeEmitterSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterSettings
{
    AudioNodeEmitterSettings_None = 0,
    AudioNodeEmitterSettings_DisableDoppler = 0x1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Audio.AudioNodeEmitterShapeKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterShapeKind
{
    AudioNodeEmitterShapeKind_Omnidirectional = 0,
    AudioNodeEmitterShapeKind_Cone = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Audio.AudioPlaybackConnectionOpenResultStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionOpenResultStatus
{
    AudioPlaybackConnectionOpenResultStatus_Success = 0,
    AudioPlaybackConnectionOpenResultStatus_RequestTimedOut = 1,
    AudioPlaybackConnectionOpenResultStatus_DeniedBySystem = 2,
    AudioPlaybackConnectionOpenResultStatus_UnknownFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Media.Audio.AudioPlaybackConnectionState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
enum __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionState
{
    AudioPlaybackConnectionState_Closed = 0,
    AudioPlaybackConnectionState_Opened = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Struct Windows.Media.Audio.MediaSourceAudioInputNodeCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CMedia_CAudio_CMediaSourceAudioInputNodeCreationStatus
{
    MediaSourceAudioInputNodeCreationStatus_Success = 0,
    MediaSourceAudioInputNodeCreationStatus_FormatNotSupported = 1,
    MediaSourceAudioInputNodeCreationStatus_NetworkError = 2,
    MediaSourceAudioInputNodeCreationStatus_UnknownFailure = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.Media.Audio.MixedRealitySpatialAudioFormatPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CMedia_CAudio_CMixedRealitySpatialAudioFormatPolicy
{
    MixedRealitySpatialAudioFormatPolicy_UseMixedRealityDefaultSpatialAudioFormat = 0,
    MixedRealitySpatialAudioFormatPolicy_UseDeviceConfigurationDefaultSpatialAudioFormat = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Media.Audio.QuantumSizeSelectionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CAudio_CQuantumSizeSelectionMode
{
    QuantumSizeSelectionMode_SystemDefault = 0,
    QuantumSizeSelectionMode_LowestLatency = 1,
    QuantumSizeSelectionMode_ClosestToDesired = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Audio.SetDefaultSpatialAudioFormatStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CMedia_CAudio_CSetDefaultSpatialAudioFormatStatus
{
    SetDefaultSpatialAudioFormatStatus_Succeeded = 0,
    SetDefaultSpatialAudioFormatStatus_AccessDenied = 1,
    SetDefaultSpatialAudioFormatStatus_LicenseExpired = 2,
    SetDefaultSpatialAudioFormatStatus_LicenseNotValidForAudioEndpoint = 3,
    SetDefaultSpatialAudioFormatStatus_NotSupportedOnAudioEndpoint = 4,
    SetDefaultSpatialAudioFormatStatus_UnknownError = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.Media.Audio.SpatialAudioModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CAudio_CSpatialAudioModel
{
    SpatialAudioModel_ObjectBased = 0,
    SpatialAudioModel_FoldDown = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioDeviceInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioDeviceInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioDeviceInputNode[] = L"Windows.Media.Audio.IAudioDeviceInputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioDeviceOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioDeviceOutputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioDeviceOutputNode[] = L"Windows.Media.Audio.IAudioDeviceOutputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Device)(__x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_get_Device(This, value) \
    ((This)->lpVtbl->get_Device(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioEffectsPackConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioEffectsPackConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioEffectsPackConfiguration[] = L"Windows.Media.Audio.IAudioEffectsPackConfiguration";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EffectsPackId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioEffectsPackStatus* value);
    HRESULT (STDMETHODCALLTYPE* add_StatusChanged)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioEffectsPackConfiguration_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StatusChanged)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_get_EffectsPackId(This, value) \
    ((This)->lpVtbl->get_EffectsPackId(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_add_StatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_StatusChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_remove_StatusChanged(This, token) \
    ((This)->lpVtbl->remove_StatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Media.Audio.IAudioEffectsPackConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioEffectsPackConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioEffectsPackConfigurationStatics[] = L"Windows.Media.Audio.IAudioEffectsPackConfigurationStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForDeviceId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This,
        HSTRING effectsPackId,
        HSTRING deviceId,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfiguration** result);
    HRESULT (STDMETHODCALLTYPE* IsDeviceIdSupported)(__x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics* This,
        HSTRING effectsPackId,
        HSTRING deviceId,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_GetForDeviceId(This, effectsPackId, deviceId, result) \
    ((This)->lpVtbl->GetForDeviceId(This, effectsPackId, deviceId, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_IsDeviceIdSupported(This, effectsPackId, deviceId, result) \
    ((This)->lpVtbl->IsDeviceIdSupported(This, effectsPackId, deviceId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioEffectsPackConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Interface Windows.Media.Audio.IAudioFileInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFileInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFileInputNode[] = L"Windows.Media.Audio.IAudioFileInputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_PlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* Seek)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan position);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_EndTime)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_EndTime)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_LoopCount)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_LoopCount)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceFile)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* add_FileCompleted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFileInputNode_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FileCompleted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_put_PlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->put_PlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_PlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->get_PlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_Seek(This, position) \
    ((This)->lpVtbl->Seek(This, position))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_EndTime(This, value) \
    ((This)->lpVtbl->get_EndTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_put_EndTime(This, value) \
    ((This)->lpVtbl->put_EndTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_LoopCount(This, value) \
    ((This)->lpVtbl->get_LoopCount(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_put_LoopCount(This, value) \
    ((This)->lpVtbl->put_LoopCount(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_get_SourceFile(This, value) \
    ((This)->lpVtbl->get_SourceFile(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_add_FileCompleted(This, handler, token) \
    ((This)->lpVtbl->add_FileCompleted(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_remove_FileCompleted(This, token) \
    ((This)->lpVtbl->remove_FileCompleted(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFileOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFileOutputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFileOutputNode[] = L"Windows.Media.Audio.IAudioFileOutputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* get_FileEncodingProfile)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* FinalizeAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode* This,
        __FIAsyncOperation_1_Windows__CMedia__CTranscoding__CTranscodeFailureReason** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_get_FileEncodingProfile(This, value) \
    ((This)->lpVtbl->get_FileEncodingProfile(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_FinalizeAsync(This, result) \
    ((This)->lpVtbl->FinalizeAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFrameCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFrameCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFrameCompletedEventArgs[] = L"Windows.Media.Audio.IAudioFrameCompletedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Frame)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CIAudioFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_get_Frame(This, value) \
    ((This)->lpVtbl->get_Frame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFrameInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFrameInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFrameInputNode[] = L"Windows.Media.Audio.IAudioFrameInputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_PlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* AddFrame)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        __x_ABI_CWindows_CMedia_CIAudioFrame* frame);
    HRESULT (STDMETHODCALLTYPE* DiscardQueuedFrames)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This);
    HRESULT (STDMETHODCALLTYPE* get_QueuedSampleCount)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* add_AudioFrameCompleted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CAudioFrameCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AudioFrameCompleted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_QuantumStarted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioFrameInputNode_Windows__CMedia__CAudio__CFrameInputNodeQuantumStartedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_QuantumStarted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_put_PlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->put_PlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_get_PlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->get_PlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_AddFrame(This, frame) \
    ((This)->lpVtbl->AddFrame(This, frame))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_DiscardQueuedFrames(This) \
    ((This)->lpVtbl->DiscardQueuedFrames(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_get_QueuedSampleCount(This, value) \
    ((This)->lpVtbl->get_QueuedSampleCount(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_add_AudioFrameCompleted(This, handler, token) \
    ((This)->lpVtbl->add_AudioFrameCompleted(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_remove_AudioFrameCompleted(This, token) \
    ((This)->lpVtbl->remove_AudioFrameCompleted(This, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_add_QuantumStarted(This, handler, token) \
    ((This)->lpVtbl->add_QuantumStarted(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_remove_QuantumStarted(This, token) \
    ((This)->lpVtbl->remove_QuantumStarted(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioFrameOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioFrameOutputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioFrameOutputNode[] = L"Windows.Media.Audio.IAudioFrameOutputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetFrame)(__x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode* This,
        __x_ABI_CWindows_CMedia_CIAudioFrame** audioFrame);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_GetFrame(This, audioFrame) \
    ((This)->lpVtbl->GetFrame(This, audioFrame))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraph[] = L"Windows.Media.Audio.IAudioGraph";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFrameInputNode)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode** frameInputNode);
    HRESULT (STDMETHODCALLTYPE* CreateFrameInputNodeWithFormat)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode** frameInputNode);
    HRESULT (STDMETHODCALLTYPE* CreateDeviceInputNodeAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateDeviceInputNodeWithFormatAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateDeviceInputNodeWithFormatOnDeviceAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* device,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFrameOutputNode)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode** frameOutputNode);
    HRESULT (STDMETHODCALLTYPE* CreateFrameOutputNodeWithFormat)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameOutputNode** frameOutputNode);
    HRESULT (STDMETHODCALLTYPE* CreateDeviceOutputNodeAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceOutputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFileInputNodeAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFileOutputNodeAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFileOutputNodeWithFileProfileAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* fileEncodingProfile,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileOutputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateSubmixNode)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode** submixNode);
    HRESULT (STDMETHODCALLTYPE* CreateSubmixNodeWithFormat)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode** submixNode);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This);
    HRESULT (STDMETHODCALLTYPE* ResetAllNodes)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This);
    HRESULT (STDMETHODCALLTYPE* add_QuantumStarted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_QuantumStarted)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_QuantumProcessed)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_QuantumProcessed)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_UnrecoverableErrorOccurred)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioGraph_Windows__CMedia__CAudio__CAudioGraphUnrecoverableErrorOccurredEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_UnrecoverableErrorOccurred)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_CompletedQuantumCount)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_EncodingProperties)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_LatencyInSamples)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryRenderDevice)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_RenderDeviceAudioProcessing)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        enum __x_ABI_CWindows_CMedia_CAudioProcessing* value);
    HRESULT (STDMETHODCALLTYPE* get_SamplesPerQuantum)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFrameInputNode(This, frameInputNode) \
    ((This)->lpVtbl->CreateFrameInputNode(This, frameInputNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFrameInputNodeWithFormat(This, encodingProperties, frameInputNode) \
    ((This)->lpVtbl->CreateFrameInputNodeWithFormat(This, encodingProperties, frameInputNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateDeviceInputNodeAsync(This, category, result) \
    ((This)->lpVtbl->CreateDeviceInputNodeAsync(This, category, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateDeviceInputNodeWithFormatAsync(This, category, encodingProperties, result) \
    ((This)->lpVtbl->CreateDeviceInputNodeWithFormatAsync(This, category, encodingProperties, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateDeviceInputNodeWithFormatOnDeviceAsync(This, category, encodingProperties, device, result) \
    ((This)->lpVtbl->CreateDeviceInputNodeWithFormatOnDeviceAsync(This, category, encodingProperties, device, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFrameOutputNode(This, frameOutputNode) \
    ((This)->lpVtbl->CreateFrameOutputNode(This, frameOutputNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFrameOutputNodeWithFormat(This, encodingProperties, frameOutputNode) \
    ((This)->lpVtbl->CreateFrameOutputNodeWithFormat(This, encodingProperties, frameOutputNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateDeviceOutputNodeAsync(This, result) \
    ((This)->lpVtbl->CreateDeviceOutputNodeAsync(This, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFileInputNodeAsync(This, file, result) \
    ((This)->lpVtbl->CreateFileInputNodeAsync(This, file, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFileOutputNodeAsync(This, file, result) \
    ((This)->lpVtbl->CreateFileOutputNodeAsync(This, file, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateFileOutputNodeWithFileProfileAsync(This, file, fileEncodingProfile, result) \
    ((This)->lpVtbl->CreateFileOutputNodeWithFileProfileAsync(This, file, fileEncodingProfile, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateSubmixNode(This, submixNode) \
    ((This)->lpVtbl->CreateSubmixNode(This, submixNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_CreateSubmixNodeWithFormat(This, encodingProperties, submixNode) \
    ((This)->lpVtbl->CreateSubmixNodeWithFormat(This, encodingProperties, submixNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_ResetAllNodes(This) \
    ((This)->lpVtbl->ResetAllNodes(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_add_QuantumStarted(This, handler, token) \
    ((This)->lpVtbl->add_QuantumStarted(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_remove_QuantumStarted(This, token) \
    ((This)->lpVtbl->remove_QuantumStarted(This, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_add_QuantumProcessed(This, handler, token) \
    ((This)->lpVtbl->add_QuantumProcessed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_remove_QuantumProcessed(This, token) \
    ((This)->lpVtbl->remove_QuantumProcessed(This, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_add_UnrecoverableErrorOccurred(This, handler, token) \
    ((This)->lpVtbl->add_UnrecoverableErrorOccurred(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_remove_UnrecoverableErrorOccurred(This, token) \
    ((This)->lpVtbl->remove_UnrecoverableErrorOccurred(This, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_get_CompletedQuantumCount(This, value) \
    ((This)->lpVtbl->get_CompletedQuantumCount(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_get_EncodingProperties(This, value) \
    ((This)->lpVtbl->get_EncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_get_LatencyInSamples(This, value) \
    ((This)->lpVtbl->get_LatencyInSamples(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_get_PrimaryRenderDevice(This, value) \
    ((This)->lpVtbl->get_PrimaryRenderDevice(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_get_RenderDeviceAudioProcessing(This, value) \
    ((This)->lpVtbl->get_RenderDeviceAudioProcessing(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_get_SamplesPerQuantum(This, value) \
    ((This)->lpVtbl->get_SamplesPerQuantum(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraph;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraph2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioGraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraph2[] = L"Windows.Media.Audio.IAudioGraph2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFrameInputNodeWithFormatAndEmitter)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* emitter,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFrameInputNode** frameInputNode);
    HRESULT (STDMETHODCALLTYPE* CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* device,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* emitter,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioDeviceInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFileInputNodeWithEmitterAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* emitter,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioFileInputNodeResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateSubmixNodeWithFormatAndEmitter)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* encodingProperties,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* emitter,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode** submixNode);
    HRESULT (STDMETHODCALLTYPE* CreateBatchUpdater)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2* This,
        __x_ABI_CWindows_CFoundation_CIClosable** updater);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_CreateFrameInputNodeWithFormatAndEmitter(This, encodingProperties, emitter, frameInputNode) \
    ((This)->lpVtbl->CreateFrameInputNodeWithFormatAndEmitter(This, encodingProperties, emitter, frameInputNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(This, category, encodingProperties, device, emitter, result) \
    ((This)->lpVtbl->CreateDeviceInputNodeWithFormatAndEmitterOnDeviceAsync(This, category, encodingProperties, device, emitter, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_CreateFileInputNodeWithEmitterAsync(This, file, emitter, result) \
    ((This)->lpVtbl->CreateFileInputNodeWithEmitterAsync(This, file, emitter, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_CreateSubmixNodeWithFormatAndEmitter(This, encodingProperties, emitter, submixNode) \
    ((This)->lpVtbl->CreateSubmixNodeWithFormatAndEmitter(This, encodingProperties, emitter, submixNode))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_CreateBatchUpdater(This, updater) \
    ((This)->lpVtbl->CreateBatchUpdater(This, updater))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraph3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraph3[] = L"Windows.Media.Audio.IAudioGraph3";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateMediaSourceAudioInputNodeAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaSource2* mediaSource,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult** operation);
    HRESULT (STDMETHODCALLTYPE* CreateMediaSourceAudioInputNodeWithEmitterAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaSource2* mediaSource,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* emitter,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateMediaSourceAudioInputNodeResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_CreateMediaSourceAudioInputNodeAsync(This, mediaSource, operation) \
    ((This)->lpVtbl->CreateMediaSourceAudioInputNodeAsync(This, mediaSource, operation))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_CreateMediaSourceAudioInputNodeWithEmitterAsync(This, mediaSource, emitter, operation) \
    ((This)->lpVtbl->CreateMediaSourceAudioInputNodeWithEmitterAsync(This, mediaSource, emitter, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraph3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphConnection[] = L"Windows.Media.Audio.IAudioGraphConnection";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Destination)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNode** value);
    HRESULT (STDMETHODCALLTYPE* put_Gain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Gain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnectionVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_get_Destination(This, value) \
    ((This)->lpVtbl->get_Destination(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_put_Gain(This, value) \
    ((This)->lpVtbl->put_Gain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_get_Gain(This, value) \
    ((This)->lpVtbl->get_Gain(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphSettings[] = L"Windows.Media.Audio.IAudioGraphSettings";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EncodingProperties)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* put_EncodingProperties)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* value);
    HRESULT (STDMETHODCALLTYPE* get_PrimaryRenderDevice)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* put_PrimaryRenderDevice)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation* value);
    HRESULT (STDMETHODCALLTYPE* get_QuantumSizeSelectionMode)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CQuantumSizeSelectionMode* value);
    HRESULT (STDMETHODCALLTYPE* put_QuantumSizeSelectionMode)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CQuantumSizeSelectionMode value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredSamplesPerQuantum)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredSamplesPerQuantum)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_AudioRenderCategory)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory* value);
    HRESULT (STDMETHODCALLTYPE* put_AudioRenderCategory)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredRenderDeviceAudioProcessing)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        enum __x_ABI_CWindows_CMedia_CAudioProcessing* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredRenderDeviceAudioProcessing)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* This,
        enum __x_ABI_CWindows_CMedia_CAudioProcessing value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_get_EncodingProperties(This, value) \
    ((This)->lpVtbl->get_EncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_put_EncodingProperties(This, value) \
    ((This)->lpVtbl->put_EncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_get_PrimaryRenderDevice(This, value) \
    ((This)->lpVtbl->get_PrimaryRenderDevice(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_put_PrimaryRenderDevice(This, value) \
    ((This)->lpVtbl->put_PrimaryRenderDevice(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_get_QuantumSizeSelectionMode(This, value) \
    ((This)->lpVtbl->get_QuantumSizeSelectionMode(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_put_QuantumSizeSelectionMode(This, value) \
    ((This)->lpVtbl->put_QuantumSizeSelectionMode(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_get_DesiredSamplesPerQuantum(This, value) \
    ((This)->lpVtbl->get_DesiredSamplesPerQuantum(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_put_DesiredSamplesPerQuantum(This, value) \
    ((This)->lpVtbl->put_DesiredSamplesPerQuantum(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_get_AudioRenderCategory(This, value) \
    ((This)->lpVtbl->get_AudioRenderCategory(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_put_AudioRenderCategory(This, value) \
    ((This)->lpVtbl->put_AudioRenderCategory(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_get_DesiredRenderDeviceAudioProcessing(This, value) \
    ((This)->lpVtbl->get_DesiredRenderDeviceAudioProcessing(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_put_DesiredRenderDeviceAudioProcessing(This, value) \
    ((This)->lpVtbl->put_DesiredRenderDeviceAudioProcessing(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphSettings2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphSettings2[] = L"Windows.Media.Audio.IAudioGraphSettings2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_MaxPlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_MaxPlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_put_MaxPlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->put_MaxPlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_get_MaxPlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->get_MaxPlaybackSpeedFactor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphSettingsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphSettingsFactory[] = L"Windows.Media.Audio.IAudioGraphSettingsFactory";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory* This,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory audioRenderCategory,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_Create(This, audioRenderCategory, value) \
    ((This)->lpVtbl->Create(This, audioRenderCategory, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettingsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraph
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphStatics[] = L"Windows.Media.Audio.IAudioGraphStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphSettings* settings,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CCreateAudioGraphResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_CreateAsync(This, settings, result) \
    ((This)->lpVtbl->CreateAsync(This, settings, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioGraphUnrecoverableErrorOccurredEventArgs[] = L"Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Error)(__x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioGraphUnrecoverableError* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_get_Error(This, value) \
    ((This)->lpVtbl->get_Error(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioGraphUnrecoverableErrorOccurredEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioInputNode[] = L"Windows.Media.Audio.IAudioInputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingConnections)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        __FIVectorView_1_Windows__CMedia__CAudio__CAudioGraphConnection** value);
    HRESULT (STDMETHODCALLTYPE* AddOutgoingConnection)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNode* destination);
    HRESULT (STDMETHODCALLTYPE* AddOutgoingConnectionWithGain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNode* destination,
        DOUBLE gain);
    HRESULT (STDMETHODCALLTYPE* RemoveOutgoingConnection)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNode* destination);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_get_OutgoingConnections(This, value) \
    ((This)->lpVtbl->get_OutgoingConnections(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_AddOutgoingConnection(This, destination) \
    ((This)->lpVtbl->AddOutgoingConnection(This, destination))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_AddOutgoingConnectionWithGain(This, destination, gain) \
    ((This)->lpVtbl->AddOutgoingConnectionWithGain(This, destination, gain))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_RemoveOutgoingConnection(This, destination) \
    ((This)->lpVtbl->RemoveOutgoingConnection(This, destination))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioInputNode2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioInputNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioInputNode2[] = L"Windows.Media.Audio.IAudioInputNode2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Emitter)(__x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_get_Emitter(This, value) \
    ((This)->lpVtbl->get_Emitter(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioInputNode2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNode[] = L"Windows.Media.Audio.IAudioNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EffectDefinitions)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        __FIVector_1_Windows__CMedia__CEffects__CIAudioEffectDefinition** value);
    HRESULT (STDMETHODCALLTYPE* put_OutgoingGain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_OutgoingGain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_EncodingProperties)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* get_ConsumeInput)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ConsumeInput)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This);
    HRESULT (STDMETHODCALLTYPE* Stop)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This);
    HRESULT (STDMETHODCALLTYPE* Reset)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This);
    HRESULT (STDMETHODCALLTYPE* DisableEffectsByDefinition)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* definition);
    HRESULT (STDMETHODCALLTYPE* EnableEffectsByDefinition)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNode* This,
        __x_ABI_CWindows_CMedia_CEffects_CIAudioEffectDefinition* definition);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_get_EffectDefinitions(This, value) \
    ((This)->lpVtbl->get_EffectDefinitions(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_put_OutgoingGain(This, value) \
    ((This)->lpVtbl->put_OutgoingGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_get_OutgoingGain(This, value) \
    ((This)->lpVtbl->get_OutgoingGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_get_EncodingProperties(This, value) \
    ((This)->lpVtbl->get_EncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_get_ConsumeInput(This, value) \
    ((This)->lpVtbl->get_ConsumeInput(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_put_ConsumeInput(This, value) \
    ((This)->lpVtbl->put_ConsumeInput(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_Stop(This) \
    ((This)->lpVtbl->Stop(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_Reset(This) \
    ((This)->lpVtbl->Reset(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_DisableEffectsByDefinition(This, definition) \
    ((This)->lpVtbl->DisableEffectsByDefinition(This, definition))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNode_EnableEffectsByDefinition(This, definition) \
    ((This)->lpVtbl->EnableEffectsByDefinition(This, definition))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitter[] = L"Windows.Media.Audio.IAudioNodeEmitter";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_Position)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_Direction)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_Direction)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_Shape)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape** value);
    HRESULT (STDMETHODCALLTYPE* get_DecayModel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel** value);
    HRESULT (STDMETHODCALLTYPE* get_Gain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Gain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DistanceScale)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_DistanceScale)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DopplerScale)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_DopplerScale)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DopplerVelocity)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_DopplerVelocity)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_IsDopplerDisabled)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_put_Position(This, value) \
    ((This)->lpVtbl->put_Position(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_Direction(This, value) \
    ((This)->lpVtbl->get_Direction(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_put_Direction(This, value) \
    ((This)->lpVtbl->put_Direction(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_Shape(This, value) \
    ((This)->lpVtbl->get_Shape(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_DecayModel(This, value) \
    ((This)->lpVtbl->get_DecayModel(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_Gain(This, value) \
    ((This)->lpVtbl->get_Gain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_put_Gain(This, value) \
    ((This)->lpVtbl->put_Gain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_DistanceScale(This, value) \
    ((This)->lpVtbl->get_DistanceScale(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_put_DistanceScale(This, value) \
    ((This)->lpVtbl->put_DistanceScale(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_DopplerScale(This, value) \
    ((This)->lpVtbl->get_DopplerScale(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_put_DopplerScale(This, value) \
    ((This)->lpVtbl->put_DopplerScale(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_DopplerVelocity(This, value) \
    ((This)->lpVtbl->get_DopplerVelocity(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_put_DopplerVelocity(This, value) \
    ((This)->lpVtbl->put_DopplerVelocity(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_get_IsDopplerDisabled(This, value) \
    ((This)->lpVtbl->get_IsDopplerDisabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitter2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitter2[] = L"Windows.Media.Audio.IAudioNodeEmitter2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SpatialAudioModel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CSpatialAudioModel* value);
    HRESULT (STDMETHODCALLTYPE* put_SpatialAudioModel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CSpatialAudioModel value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_get_SpatialAudioModel(This, value) \
    ((This)->lpVtbl->get_SpatialAudioModel(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_put_SpatialAudioModel(This, value) \
    ((This)->lpVtbl->put_SpatialAudioModel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterConeProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterConeProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterConeProperties[] = L"Windows.Media.Audio.IAudioNodeEmitterConeProperties";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConePropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InnerAngle)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_OuterAngle)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_OuterAngleGain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConePropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConePropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_get_InnerAngle(This, value) \
    ((This)->lpVtbl->get_InnerAngle(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_get_OuterAngle(This, value) \
    ((This)->lpVtbl->get_OuterAngle(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_get_OuterAngleGain(This, value) \
    ((This)->lpVtbl->get_OuterAngleGain(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterDecayModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterDecayModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterDecayModel[] = L"Windows.Media.Audio.IAudioNodeEmitterDecayModel";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterDecayKind* value);
    HRESULT (STDMETHODCALLTYPE* get_MinGain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_MaxGain)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_NaturalProperties)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_get_MinGain(This, value) \
    ((This)->lpVtbl->get_MinGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_get_MaxGain(This, value) \
    ((This)->lpVtbl->get_MaxGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_get_NaturalProperties(This, value) \
    ((This)->lpVtbl->get_NaturalProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterDecayModel
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterDecayModelStatics[] = L"Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateNatural)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This,
        DOUBLE minGain,
        DOUBLE maxGain,
        DOUBLE unityGainDistance,
        DOUBLE cutoffDistance,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel** decayModel);
    HRESULT (STDMETHODCALLTYPE* CreateCustom)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics* This,
        DOUBLE minGain,
        DOUBLE maxGain,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel** decayModel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_CreateNatural(This, minGain, maxGain, unityGainDistance, cutoffDistance, decayModel) \
    ((This)->lpVtbl->CreateNatural(This, minGain, maxGain, unityGainDistance, cutoffDistance, decayModel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_CreateCustom(This, minGain, maxGain, decayModel) \
    ((This)->lpVtbl->CreateCustom(This, minGain, maxGain, decayModel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModelStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitter
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterFactory[] = L"Windows.Media.Audio.IAudioNodeEmitterFactory";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAudioNodeEmitter)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* shape,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterDecayModel* decayModel,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterSettings settings,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitter** emitter);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_CreateAudioNodeEmitter(This, shape, decayModel, settings, emitter) \
    ((This)->lpVtbl->CreateAudioNodeEmitter(This, shape, decayModel, settings, emitter))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterNaturalDecayModelProperties[] = L"Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UnityGainDistance)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_CutoffDistance)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_get_UnityGainDistance(This, value) \
    ((This)->lpVtbl->get_UnityGainDistance(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_get_CutoffDistance(This, value) \
    ((This)->lpVtbl->get_CutoffDistance(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterNaturalDecayModelProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterShape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterShape[] = L"Windows.Media.Audio.IAudioNodeEmitterShape";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Kind)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioNodeEmitterShapeKind* value);
    HRESULT (STDMETHODCALLTYPE* get_ConeProperties)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterConeProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_get_Kind(This, value) \
    ((This)->lpVtbl->get_Kind(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_get_ConeProperties(This, value) \
    ((This)->lpVtbl->get_ConeProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeEmitterShapeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeEmitterShape
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeEmitterShapeStatics[] = L"Windows.Media.Audio.IAudioNodeEmitterShapeStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateCone)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This,
        DOUBLE innerAngle,
        DOUBLE outerAngle,
        DOUBLE outerAngleGain,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape** shape);
    HRESULT (STDMETHODCALLTYPE* CreateOmnidirectional)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShape** shape);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_CreateCone(This, innerAngle, outerAngle, outerAngleGain, shape) \
    ((This)->lpVtbl->CreateCone(This, innerAngle, outerAngle, outerAngleGain, shape))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_CreateOmnidirectional(This, shape) \
    ((This)->lpVtbl->CreateOmnidirectional(This, shape))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeEmitterShapeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioNodeListener
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeListener[] = L"Windows.Media.Audio.IAudioNodeListener";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_Position)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);
    HRESULT (STDMETHODCALLTYPE* get_Orientation)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion* value);
    HRESULT (STDMETHODCALLTYPE* put_Orientation)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CQuaternion value);
    HRESULT (STDMETHODCALLTYPE* get_SpeedOfSound)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_SpeedOfSound)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DopplerVelocity)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3* value);
    HRESULT (STDMETHODCALLTYPE* put_DopplerVelocity)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* This,
        struct __x_ABI_CWindows_CFoundation_CNumerics_CVector3 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListenerVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_put_Position(This, value) \
    ((This)->lpVtbl->put_Position(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_get_Orientation(This, value) \
    ((This)->lpVtbl->get_Orientation(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_put_Orientation(This, value) \
    ((This)->lpVtbl->put_Orientation(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_get_SpeedOfSound(This, value) \
    ((This)->lpVtbl->get_SpeedOfSound(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_put_SpeedOfSound(This, value) \
    ((This)->lpVtbl->put_SpeedOfSound(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_get_DopplerVelocity(This, value) \
    ((This)->lpVtbl->get_DopplerVelocity(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_put_DopplerVelocity(This, value) \
    ((This)->lpVtbl->put_DopplerVelocity(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioNodeWithListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioNode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioNodeWithListener[] = L"Windows.Media.Audio.IAudioNodeWithListener";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListenerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Listener)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener* value);
    HRESULT (STDMETHODCALLTYPE* get_Listener)(__x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeListener** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListenerVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListenerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_put_Listener(This, value) \
    ((This)->lpVtbl->put_Listener(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_get_Listener(This, value) \
    ((This)->lpVtbl->get_Listener(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioNodeWithListener_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Audio.IAudioPlaybackConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioPlaybackConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioPlaybackConnection[] = L"Windows.Media.Audio.IAudioPlaybackConnection";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionState* value);
    HRESULT (STDMETHODCALLTYPE* Open)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult** result);
    HRESULT (STDMETHODCALLTYPE* OpenAsync)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CAudioPlaybackConnectionOpenResult** operation);
    HRESULT (STDMETHODCALLTYPE* add_StateChanged)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioPlaybackConnection_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StateChanged)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_Open(This, result) \
    ((This)->lpVtbl->Open(This, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_OpenAsync(This, operation) \
    ((This)->lpVtbl->OpenAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_add_StateChanged(This, handler, token) \
    ((This)->lpVtbl->add_StateChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_remove_StateChanged(This, token) \
    ((This)->lpVtbl->remove_StateChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Audio.IAudioPlaybackConnectionOpenResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioPlaybackConnectionOpenResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioPlaybackConnectionOpenResult[] = L"Windows.Media.Audio.IAudioPlaybackConnectionOpenResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioPlaybackConnectionOpenResultStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionOpenResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Audio.IAudioPlaybackConnectionStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioPlaybackConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioPlaybackConnectionStatics[] = L"Windows.Media.Audio.IAudioPlaybackConnectionStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* TryCreateFromId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics* This,
        HSTRING id,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnection** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_GetDeviceSelector(This, result) \
    ((This)->lpVtbl->GetDeviceSelector(This, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_TryCreateFromId(This, id, result) \
    ((This)->lpVtbl->TryCreateFromId(This, id, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioPlaybackConnectionStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Audio.IAudioStateMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioStateMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioStateMonitor[] = L"Windows.Media.Audio.IAudioStateMonitor";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_SoundLevelChanged)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CAudioStateMonitor_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SoundLevelChanged)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_SoundLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor* This,
        enum __x_ABI_CWindows_CMedia_CSoundLevel* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_add_SoundLevelChanged(This, handler, token) \
    ((This)->lpVtbl->add_SoundLevelChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_remove_SoundLevelChanged(This, token) \
    ((This)->lpVtbl->remove_SoundLevelChanged(This, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_get_SoundLevel(This, value) \
    ((This)->lpVtbl->get_SoundLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IAudioStateMonitorStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.AudioStateMonitor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IAudioStateMonitorStatics[] = L"Windows.Media.Audio.IAudioStateMonitorStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateForRenderMonitoring)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForRenderMonitoringWithCategory)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory category,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForRenderMonitoringWithCategoryAndDeviceRole)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory category,
        enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole role,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForRenderMonitoringWithCategoryAndDeviceId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        enum __x_ABI_CWindows_CMedia_CRender_CAudioRenderCategory category,
        HSTRING deviceId,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForCaptureMonitoring)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForCaptureMonitoringWithCategory)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForCaptureMonitoringWithCategoryAndDeviceRole)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        enum __x_ABI_CWindows_CMedia_CDevices_CAudioDeviceRole role,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);
    HRESULT (STDMETHODCALLTYPE* CreateForCaptureMonitoringWithCategoryAndDeviceId)(__x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaCategory category,
        HSTRING deviceId,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitor** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForRenderMonitoring(This, result) \
    ((This)->lpVtbl->CreateForRenderMonitoring(This, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForRenderMonitoringWithCategory(This, category, result) \
    ((This)->lpVtbl->CreateForRenderMonitoringWithCategory(This, category, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForRenderMonitoringWithCategoryAndDeviceRole(This, category, role, result) \
    ((This)->lpVtbl->CreateForRenderMonitoringWithCategoryAndDeviceRole(This, category, role, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForRenderMonitoringWithCategoryAndDeviceId(This, category, deviceId, result) \
    ((This)->lpVtbl->CreateForRenderMonitoringWithCategoryAndDeviceId(This, category, deviceId, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForCaptureMonitoring(This, result) \
    ((This)->lpVtbl->CreateForCaptureMonitoring(This, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForCaptureMonitoringWithCategory(This, category, result) \
    ((This)->lpVtbl->CreateForCaptureMonitoringWithCategory(This, category, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForCaptureMonitoringWithCategoryAndDeviceRole(This, category, role, result) \
    ((This)->lpVtbl->CreateForCaptureMonitoringWithCategoryAndDeviceRole(This, category, role, result))

#define __x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_CreateForCaptureMonitoringWithCategoryAndDeviceId(This, category, deviceId, result) \
    ((This)->lpVtbl->CreateForCaptureMonitoringWithCategoryAndDeviceId(This, category, deviceId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIAudioStateMonitorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceInputNodeResult[] = L"Windows.Media.Audio.ICreateAudioDeviceInputNodeResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioDeviceNodeCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInputNode)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceInputNode** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_get_DeviceInputNode(This, value) \
    ((This)->lpVtbl->get_DeviceInputNode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceInputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceInputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceOutputNodeResult[] = L"Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioDeviceNodeCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceOutputNode)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioDeviceOutputNode** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_get_DeviceOutputNode(This, value) \
    ((This)->lpVtbl->get_DeviceOutputNode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioDeviceOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioDeviceOutputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioDeviceOutputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileInputNodeResult[] = L"Windows.Media.Audio.ICreateAudioFileInputNodeResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioFileNodeCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_FileInputNode)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFileInputNode** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_get_FileInputNode(This, value) \
    ((This)->lpVtbl->get_FileInputNode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileInputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileInputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioFileInputNodeResult2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileInputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileOutputNodeResult[] = L"Windows.Media.Audio.ICreateAudioFileOutputNodeResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioFileNodeCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_FileOutputNode)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioFileOutputNode** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_get_FileOutputNode(This, value) \
    ((This)->lpVtbl->get_FileOutputNode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioFileOutputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioFileOutputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioFileOutputNodeResult2[] = L"Windows.Media.Audio.ICreateAudioFileOutputNodeResult2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioFileOutputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioGraphResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioGraphResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioGraphResult[] = L"Windows.Media.Audio.ICreateAudioGraphResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CAudioGraphCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Graph)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_get_Graph(This, value) \
    ((This)->lpVtbl->get_Graph(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ICreateAudioGraphResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateAudioGraphResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateAudioGraphResult2[] = L"Windows.Media.Audio.ICreateAudioGraphResult2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateAudioGraphResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateMediaSourceAudioInputNodeResult[] = L"Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CMediaSourceAudioInputNodeCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Node)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult* This,
        __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_get_Node(This, value) \
    ((This)->lpVtbl->get_Node(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ICreateMediaSourceAudioInputNodeResult2[] = L"Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CICreateMediaSourceAudioInputNodeResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.IEchoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EchoEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEchoEffectDefinition[] = L"Windows.Media.Audio.IEchoEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WetDryMix)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_WetDryMix)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Feedback)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Feedback)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Delay)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Delay)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_put_WetDryMix(This, value) \
    ((This)->lpVtbl->put_WetDryMix(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_get_WetDryMix(This, value) \
    ((This)->lpVtbl->get_WetDryMix(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_put_Feedback(This, value) \
    ((This)->lpVtbl->put_Feedback(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_get_Feedback(This, value) \
    ((This)->lpVtbl->get_Feedback(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_put_Delay(This, value) \
    ((This)->lpVtbl->put_Delay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_get_Delay(This, value) \
    ((This)->lpVtbl->get_Delay(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEchoEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EchoEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEchoEffectDefinitionFactory[] = L"Windows.Media.Audio.IEchoEffectDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* audioGraph,
        __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_Create(This, audioGraph, value) \
    ((This)->lpVtbl->Create(This, audioGraph, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEchoEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEqualizerBand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EqualizerBand
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEqualizerBand[] = L"Windows.Media.Audio.IEqualizerBand";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBandVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bandwidth)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Bandwidth)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_FrequencyCenter)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_FrequencyCenter)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Gain)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Gain)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand* This,
        DOUBLE value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBandVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBandVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_get_Bandwidth(This, value) \
    ((This)->lpVtbl->get_Bandwidth(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_put_Bandwidth(This, value) \
    ((This)->lpVtbl->put_Bandwidth(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_get_FrequencyCenter(This, value) \
    ((This)->lpVtbl->get_FrequencyCenter(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_put_FrequencyCenter(This, value) \
    ((This)->lpVtbl->put_FrequencyCenter(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_get_Gain(This, value) \
    ((This)->lpVtbl->get_Gain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_put_Gain(This, value) \
    ((This)->lpVtbl->put_Gain(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerBand_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEqualizerEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EqualizerEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEqualizerEffectDefinition[] = L"Windows.Media.Audio.IEqualizerEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Bands)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition* This,
        __FIVectorView_1_Windows__CMedia__CAudio__CEqualizerBand** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_get_Bands(This, value) \
    ((This)->lpVtbl->get_Bands(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IEqualizerEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.EqualizerEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IEqualizerEffectDefinitionFactory[] = L"Windows.Media.Audio.IEqualizerEffectDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* audioGraph,
        __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_Create(This, audioGraph, value) \
    ((This)->lpVtbl->Create(This, audioGraph, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIEqualizerEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IFrameInputNodeQuantumStartedEventArgs[] = L"Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequiredSamples)(__x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_get_RequiredSamples(This, value) \
    ((This)->lpVtbl->get_RequiredSamples(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIFrameInputNodeQuantumStartedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ILimiterEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.LimiterEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ILimiterEffectDefinition[] = L"Windows.Media.Audio.ILimiterEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Release)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Release)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Loudness)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Loudness)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_put_Release(This, value) \
    ((This)->lpVtbl->put_Release(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_get_Release(This, value) \
    ((This)->lpVtbl->get_Release(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_put_Loudness(This, value) \
    ((This)->lpVtbl->put_Loudness(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_get_Loudness(This, value) \
    ((This)->lpVtbl->get_Loudness(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ILimiterEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.LimiterEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ILimiterEffectDefinitionFactory[] = L"Windows.Media.Audio.ILimiterEffectDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* audioGraph,
        __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_Create(This, audioGraph, value) \
    ((This)->lpVtbl->Create(This, audioGraph, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CILimiterEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IMediaSourceAudioInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.MediaSourceAudioInputNode
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Audio.IAudioInputNode
 *     Windows.Media.Audio.IAudioNode
 *     Windows.Foundation.IClosable
 *     Windows.Media.Audio.IAudioInputNode2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IMediaSourceAudioInputNode[] = L"Windows.Media.Audio.IMediaSourceAudioInputNode";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNodeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_PlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_PlaybackSpeedFactor)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* Seek)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan position);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_StartTime)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_EndTime)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_EndTime)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_LoopCount)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_LoopCount)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaSource)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaSource2** value);
    HRESULT (STDMETHODCALLTYPE* add_MediaSourceCompleted)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CMediaSourceAudioInputNode_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_MediaSourceCompleted)(__x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNodeVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNodeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_put_PlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->put_PlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_PlaybackSpeedFactor(This, value) \
    ((This)->lpVtbl->get_PlaybackSpeedFactor(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_Seek(This, position) \
    ((This)->lpVtbl->Seek(This, position))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_put_StartTime(This, value) \
    ((This)->lpVtbl->put_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_EndTime(This, value) \
    ((This)->lpVtbl->get_EndTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_put_EndTime(This, value) \
    ((This)->lpVtbl->put_EndTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_LoopCount(This, value) \
    ((This)->lpVtbl->get_LoopCount(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_put_LoopCount(This, value) \
    ((This)->lpVtbl->put_LoopCount(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_get_MediaSource(This, value) \
    ((This)->lpVtbl->get_MediaSource(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_add_MediaSourceCompleted(This, handler, token) \
    ((This)->lpVtbl->add_MediaSourceCompleted(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_remove_MediaSourceCompleted(This, token) \
    ((This)->lpVtbl->remove_MediaSourceCompleted(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIMediaSourceAudioInputNode_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Audio.IReverbEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.ReverbEffectDefinition
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Effects.IAudioEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IReverbEffectDefinition[] = L"Windows.Media.Audio.IReverbEffectDefinition";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_WetDryMix)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_WetDryMix)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_ReflectionsDelay)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ReflectionsDelay)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ReverbDelay)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_ReverbDelay)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_RearDelay)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_RearDelay)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_PositionLeft)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_PositionLeft)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_PositionRight)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_PositionRight)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_PositionMatrixLeft)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_PositionMatrixLeft)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_PositionMatrixRight)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_PositionMatrixRight)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_EarlyDiffusion)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_EarlyDiffusion)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_LateDiffusion)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_LateDiffusion)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_LowEQGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_LowEQGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_LowEQCutoff)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_LowEQCutoff)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_HighEQGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_HighEQGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_HighEQCutoff)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE value);
    HRESULT (STDMETHODCALLTYPE* get_HighEQCutoff)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* put_RoomFilterFreq)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RoomFilterFreq)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RoomFilterMain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RoomFilterMain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RoomFilterHF)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RoomFilterHF)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_ReflectionsGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_ReflectionsGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_ReverbGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_ReverbGain)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_DecayTime)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_DecayTime)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_Density)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_Density)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_RoomSize)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE value);
    HRESULT (STDMETHODCALLTYPE* get_RoomSize)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* put_DisableLateField)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DisableLateField)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_WetDryMix(This, value) \
    ((This)->lpVtbl->put_WetDryMix(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_WetDryMix(This, value) \
    ((This)->lpVtbl->get_WetDryMix(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_ReflectionsDelay(This, value) \
    ((This)->lpVtbl->put_ReflectionsDelay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_ReflectionsDelay(This, value) \
    ((This)->lpVtbl->get_ReflectionsDelay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_ReverbDelay(This, value) \
    ((This)->lpVtbl->put_ReverbDelay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_ReverbDelay(This, value) \
    ((This)->lpVtbl->get_ReverbDelay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_RearDelay(This, value) \
    ((This)->lpVtbl->put_RearDelay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_RearDelay(This, value) \
    ((This)->lpVtbl->get_RearDelay(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_PositionLeft(This, value) \
    ((This)->lpVtbl->put_PositionLeft(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_PositionLeft(This, value) \
    ((This)->lpVtbl->get_PositionLeft(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_PositionRight(This, value) \
    ((This)->lpVtbl->put_PositionRight(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_PositionRight(This, value) \
    ((This)->lpVtbl->get_PositionRight(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_PositionMatrixLeft(This, value) \
    ((This)->lpVtbl->put_PositionMatrixLeft(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_PositionMatrixLeft(This, value) \
    ((This)->lpVtbl->get_PositionMatrixLeft(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_PositionMatrixRight(This, value) \
    ((This)->lpVtbl->put_PositionMatrixRight(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_PositionMatrixRight(This, value) \
    ((This)->lpVtbl->get_PositionMatrixRight(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_EarlyDiffusion(This, value) \
    ((This)->lpVtbl->put_EarlyDiffusion(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_EarlyDiffusion(This, value) \
    ((This)->lpVtbl->get_EarlyDiffusion(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_LateDiffusion(This, value) \
    ((This)->lpVtbl->put_LateDiffusion(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_LateDiffusion(This, value) \
    ((This)->lpVtbl->get_LateDiffusion(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_LowEQGain(This, value) \
    ((This)->lpVtbl->put_LowEQGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_LowEQGain(This, value) \
    ((This)->lpVtbl->get_LowEQGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_LowEQCutoff(This, value) \
    ((This)->lpVtbl->put_LowEQCutoff(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_LowEQCutoff(This, value) \
    ((This)->lpVtbl->get_LowEQCutoff(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_HighEQGain(This, value) \
    ((This)->lpVtbl->put_HighEQGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_HighEQGain(This, value) \
    ((This)->lpVtbl->get_HighEQGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_HighEQCutoff(This, value) \
    ((This)->lpVtbl->put_HighEQCutoff(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_HighEQCutoff(This, value) \
    ((This)->lpVtbl->get_HighEQCutoff(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_RoomFilterFreq(This, value) \
    ((This)->lpVtbl->put_RoomFilterFreq(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_RoomFilterFreq(This, value) \
    ((This)->lpVtbl->get_RoomFilterFreq(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_RoomFilterMain(This, value) \
    ((This)->lpVtbl->put_RoomFilterMain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_RoomFilterMain(This, value) \
    ((This)->lpVtbl->get_RoomFilterMain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_RoomFilterHF(This, value) \
    ((This)->lpVtbl->put_RoomFilterHF(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_RoomFilterHF(This, value) \
    ((This)->lpVtbl->get_RoomFilterHF(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_ReflectionsGain(This, value) \
    ((This)->lpVtbl->put_ReflectionsGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_ReflectionsGain(This, value) \
    ((This)->lpVtbl->get_ReflectionsGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_ReverbGain(This, value) \
    ((This)->lpVtbl->put_ReverbGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_ReverbGain(This, value) \
    ((This)->lpVtbl->get_ReverbGain(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_DecayTime(This, value) \
    ((This)->lpVtbl->put_DecayTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_DecayTime(This, value) \
    ((This)->lpVtbl->get_DecayTime(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_Density(This, value) \
    ((This)->lpVtbl->put_Density(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_Density(This, value) \
    ((This)->lpVtbl->get_Density(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_RoomSize(This, value) \
    ((This)->lpVtbl->put_RoomSize(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_RoomSize(This, value) \
    ((This)->lpVtbl->get_RoomSize(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_put_DisableLateField(This, value) \
    ((This)->lpVtbl->put_DisableLateField(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_get_DisableLateField(This, value) \
    ((This)->lpVtbl->get_DisableLateField(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.IReverbEffectDefinitionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.ReverbEffectDefinition
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_IReverbEffectDefinitionFactory[] = L"Windows.Media.Audio.IReverbEffectDefinitionFactory";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Create)(__x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory* This,
        __x_ABI_CWindows_CMedia_CAudio_CIAudioGraph* audioGraph,
        __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinition** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_Create(This, audioGraph, value) \
    ((This)->lpVtbl->Create(This, audioGraph, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CIReverbEffectDefinitionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SetDefaultSpatialAudioFormatResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISetDefaultSpatialAudioFormatResult[] = L"Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CSetDefaultSpatialAudioFormatStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResultVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISetDefaultSpatialAudioFormatResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioDeviceConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioDeviceConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioDeviceConfiguration[] = L"Windows.Media.Audio.ISpatialAudioDeviceConfiguration";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DeviceId)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSpatialAudioSupported)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* IsSpatialAudioFormatSupported)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        HSTRING subtype,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* get_ActiveSpatialAudioFormat)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DefaultSpatialAudioFormat)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* SetDefaultSpatialAudioFormatAsync)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        HSTRING subtype,
        __FIAsyncOperation_1_Windows__CMedia__CAudio__CSetDefaultSpatialAudioFormatResult** operation);
    HRESULT (STDMETHODCALLTYPE* add_ConfigurationChanged)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        __FITypedEventHandler_2_Windows__CMedia__CAudio__CSpatialAudioDeviceConfiguration_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ConfigurationChanged)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_get_DeviceId(This, value) \
    ((This)->lpVtbl->get_DeviceId(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_get_IsSpatialAudioSupported(This, value) \
    ((This)->lpVtbl->get_IsSpatialAudioSupported(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_IsSpatialAudioFormatSupported(This, subtype, result) \
    ((This)->lpVtbl->IsSpatialAudioFormatSupported(This, subtype, result))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_get_ActiveSpatialAudioFormat(This, value) \
    ((This)->lpVtbl->get_ActiveSpatialAudioFormat(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_get_DefaultSpatialAudioFormat(This, value) \
    ((This)->lpVtbl->get_DefaultSpatialAudioFormat(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_SetDefaultSpatialAudioFormatAsync(This, subtype, operation) \
    ((This)->lpVtbl->SetDefaultSpatialAudioFormatAsync(This, subtype, operation))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_add_ConfigurationChanged(This, handler, token) \
    ((This)->lpVtbl->add_ConfigurationChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_remove_ConfigurationChanged(This, token) \
    ((This)->lpVtbl->remove_ConfigurationChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioDeviceConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioDeviceConfigurationStatics[] = L"Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForDeviceId)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics* This,
        HSTRING deviceId,
        __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfiguration** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_GetForDeviceId(This, deviceId, result) \
    ((This)->lpVtbl->GetForDeviceId(This, deviceId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioDeviceConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatConfiguration[] = L"Windows.Media.Audio.ISpatialAudioFormatConfiguration";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportLicenseChangedAsync)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        HSTRING subtype,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportConfigurationChangedAsync)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        HSTRING subtype,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* get_MixedRealityExclusiveModePolicy)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CMixedRealitySpatialAudioFormatPolicy* value);
    HRESULT (STDMETHODCALLTYPE* put_MixedRealityExclusiveModePolicy)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration* This,
        enum __x_ABI_CWindows_CMedia_CAudio_CMixedRealitySpatialAudioFormatPolicy value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_ReportLicenseChangedAsync(This, subtype, operation) \
    ((This)->lpVtbl->ReportLicenseChangedAsync(This, subtype, operation))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_ReportConfigurationChangedAsync(This, subtype, operation) \
    ((This)->lpVtbl->ReportConfigurationChangedAsync(This, subtype, operation))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_get_MixedRealityExclusiveModePolicy(This, value) \
    ((This)->lpVtbl->get_MixedRealityExclusiveModePolicy(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_put_MixedRealityExclusiveModePolicy(This, value) \
    ((This)->lpVtbl->put_MixedRealityExclusiveModePolicy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatConfiguration
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatConfigurationStatics[] = L"Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics* This,
        __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfiguration** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_GetDefault(This, result) \
    ((This)->lpVtbl->GetDefault(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatConfigurationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatSubtype
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatSubtypeStatics[] = L"Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WindowsSonic)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DolbyAtmosForHeadphones)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DolbyAtmosForHomeTheater)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DolbyAtmosForSpeakers)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DTSHeadphoneX)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DTSXUltra)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_get_WindowsSonic(This, value) \
    ((This)->lpVtbl->get_WindowsSonic(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_get_DolbyAtmosForHeadphones(This, value) \
    ((This)->lpVtbl->get_DolbyAtmosForHeadphones(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_get_DolbyAtmosForHomeTheater(This, value) \
    ((This)->lpVtbl->get_DolbyAtmosForHomeTheater(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_get_DolbyAtmosForSpeakers(This, value) \
    ((This)->lpVtbl->get_DolbyAtmosForSpeakers(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_get_DTSHeadphoneX(This, value) \
    ((This)->lpVtbl->get_DTSHeadphoneX(This, value))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_get_DTSXUltra(This, value) \
    ((This)->lpVtbl->get_DTSXUltra(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 12.0
 *
 * Interface is a part of the implementation of type Windows.Media.Audio.SpatialAudioFormatSubtype
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
#if !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Audio_ISpatialAudioFormatSubtypeStatics2[] = L"Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2";
typedef struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DTSXForHomeTheater)(__x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_get_DTSXForHomeTheater(This, value) \
    ((This)->lpVtbl->get_DTSXForHomeTheater(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CAudio_CISpatialAudioFormatSubtypeStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000

/*
 *
 * Class Windows.Media.Audio.AudioDeviceInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioDeviceInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioDeviceInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioDeviceInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioDeviceInputNode[] = L"Windows.Media.Audio.AudioDeviceInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioDeviceOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioDeviceOutputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioNodeWithListener
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioDeviceOutputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioDeviceOutputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioDeviceOutputNode[] = L"Windows.Media.Audio.AudioDeviceOutputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioEffectsPackConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 16.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioEffectsPackConfigurationStatics interface starting with version 16.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioEffectsPackConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioEffectsPackConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioEffectsPackConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioEffectsPackConfiguration[] = L"Windows.Media.Audio.AudioEffectsPackConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x100000

/*
 *
 * Class Windows.Media.Audio.AudioFileInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFileInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFileInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFileInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFileInputNode[] = L"Windows.Media.Audio.AudioFileInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFileOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFileOutputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFileOutputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFileOutputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFileOutputNode[] = L"Windows.Media.Audio.AudioFileOutputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFrameCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFrameCompletedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFrameCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFrameCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFrameCompletedEventArgs[] = L"Windows.Media.Audio.AudioFrameCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFrameInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFrameInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFrameInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFrameInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFrameInputNode[] = L"Windows.Media.Audio.AudioFrameInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioFrameOutputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioFrameOutputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioFrameOutputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioFrameOutputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioFrameOutputNode[] = L"Windows.Media.Audio.AudioFrameOutputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraph
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioGraphStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraph ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioGraph2
 *    Windows.Media.Audio.IAudioGraph3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraph_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraph_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraph[] = L"Windows.Media.Audio.AudioGraph";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraphBatchUpdater
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.IClosable ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphBatchUpdater_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphBatchUpdater_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphBatchUpdater[] = L"Windows.Media.Audio.AudioGraphBatchUpdater";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioGraphConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraphConnection ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphConnection[] = L"Windows.Media.Audio.AudioGraphConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraphSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IAudioGraphSettingsFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraphSettings ** Default Interface **
 *    Windows.Media.Audio.IAudioGraphSettings2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphSettings[] = L"Windows.Media.Audio.AudioGraphSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioGraphUnrecoverableErrorOccurredEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioGraphUnrecoverableErrorOccurredEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioGraphUnrecoverableErrorOccurredEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioGraphUnrecoverableErrorOccurredEventArgs[] = L"Windows.Media.Audio.AudioGraphUnrecoverableErrorOccurredEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitter
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Media.Audio.IAudioNodeEmitterFactory interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitter ** Default Interface **
 *    Windows.Media.Audio.IAudioNodeEmitter2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitter_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitter_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitter[] = L"Windows.Media.Audio.AudioNodeEmitter";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterConeProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterConeProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterConeProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterConeProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterConeProperties[] = L"Windows.Media.Audio.AudioNodeEmitterConeProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterDecayModel
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioNodeEmitterDecayModelStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterDecayModel ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterDecayModel_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterDecayModel_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterDecayModel[] = L"Windows.Media.Audio.AudioNodeEmitterDecayModel";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterNaturalDecayModelProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterNaturalDecayModelProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterNaturalDecayModelProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterNaturalDecayModelProperties[] = L"Windows.Media.Audio.AudioNodeEmitterNaturalDecayModelProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeEmitterShape
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioNodeEmitterShapeStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeEmitterShape ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterShape_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeEmitterShape_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeEmitterShape[] = L"Windows.Media.Audio.AudioNodeEmitterShape";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioNodeListener
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioNodeListener ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioNodeListener_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioNodeListener_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioNodeListener[] = L"Windows.Media.Audio.AudioNodeListener";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Audio.AudioPlaybackConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioPlaybackConnectionStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioPlaybackConnection ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnection_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioPlaybackConnection[] = L"Windows.Media.Audio.AudioPlaybackConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Media.Audio.AudioPlaybackConnectionOpenResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioPlaybackConnectionOpenResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnectionOpenResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioPlaybackConnectionOpenResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioPlaybackConnectionOpenResult[] = L"Windows.Media.Audio.AudioPlaybackConnectionOpenResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Media.Audio.AudioStateMonitor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.IAudioStateMonitorStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioStateMonitor ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioStateMonitor_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioStateMonitor_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioStateMonitor[] = L"Windows.Media.Audio.AudioStateMonitor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Audio.AudioSubmixNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IAudioInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *    Windows.Media.Audio.IAudioInputNode2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_AudioSubmixNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_AudioSubmixNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_AudioSubmixNode[] = L"Windows.Media.Audio.AudioSubmixNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioDeviceInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioDeviceInputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioDeviceInputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceInputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceInputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioDeviceInputNodeResult[] = L"Windows.Media.Audio.CreateAudioDeviceInputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioDeviceOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioDeviceOutputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceOutputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioDeviceOutputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioDeviceOutputNodeResult[] = L"Windows.Media.Audio.CreateAudioDeviceOutputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioFileInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioFileInputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioFileInputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileInputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileInputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioFileInputNodeResult[] = L"Windows.Media.Audio.CreateAudioFileInputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioFileOutputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioFileOutputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioFileOutputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileOutputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioFileOutputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioFileOutputNodeResult[] = L"Windows.Media.Audio.CreateAudioFileOutputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateAudioGraphResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateAudioGraphResult ** Default Interface **
 *    Windows.Media.Audio.ICreateAudioGraphResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateAudioGraphResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateAudioGraphResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateAudioGraphResult[] = L"Windows.Media.Audio.CreateAudioGraphResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult ** Default Interface **
 *    Windows.Media.Audio.ICreateMediaSourceAudioInputNodeResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Audio_CreateMediaSourceAudioInputNodeResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_CreateMediaSourceAudioInputNodeResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_CreateMediaSourceAudioInputNodeResult[] = L"Windows.Media.Audio.CreateMediaSourceAudioInputNodeResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Audio.EchoEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IEchoEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IEchoEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_EchoEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_EchoEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_EchoEffectDefinition[] = L"Windows.Media.Audio.EchoEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.EqualizerBand
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IEqualizerBand ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_EqualizerBand_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_EqualizerBand_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_EqualizerBand[] = L"Windows.Media.Audio.EqualizerBand";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.EqualizerEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IEqualizerEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IEqualizerEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_EqualizerEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_EqualizerEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_EqualizerEffectDefinition[] = L"Windows.Media.Audio.EqualizerEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IFrameInputNodeQuantumStartedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_FrameInputNodeQuantumStartedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_FrameInputNodeQuantumStartedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_FrameInputNodeQuantumStartedEventArgs[] = L"Windows.Media.Audio.FrameInputNodeQuantumStartedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.LimiterEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.ILimiterEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ILimiterEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_LimiterEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_LimiterEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_LimiterEffectDefinition[] = L"Windows.Media.Audio.LimiterEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.MediaSourceAudioInputNode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IMediaSourceAudioInputNode ** Default Interface **
 *    Windows.Media.Audio.IAudioInputNode2
 *    Windows.Media.Audio.IAudioInputNode
 *    Windows.Media.Audio.IAudioNode
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Audio_MediaSourceAudioInputNode_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_MediaSourceAudioInputNode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_MediaSourceAudioInputNode[] = L"Windows.Media.Audio.MediaSourceAudioInputNode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Audio.ReverbEffectDefinition
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Audio.IReverbEffectDefinitionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.IReverbEffectDefinition ** Default Interface **
 *    Windows.Media.Effects.IAudioEffectDefinition
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Audio_ReverbEffectDefinition_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_ReverbEffectDefinition_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_ReverbEffectDefinition[] = L"Windows.Media.Audio.ReverbEffectDefinition";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Audio.SetDefaultSpatialAudioFormatResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ISetDefaultSpatialAudioFormatResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SetDefaultSpatialAudioFormatResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SetDefaultSpatialAudioFormatResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SetDefaultSpatialAudioFormatResult[] = L"Windows.Media.Audio.SetDefaultSpatialAudioFormatResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Audio.SpatialAudioDeviceConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioDeviceConfigurationStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ISpatialAudioDeviceConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SpatialAudioDeviceConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SpatialAudioDeviceConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SpatialAudioDeviceConfiguration[] = L"Windows.Media.Audio.SpatialAudioDeviceConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Audio.SpatialAudioFormatConfiguration
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioFormatConfigurationStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Audio.ISpatialAudioFormatConfiguration ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatConfiguration_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatConfiguration_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SpatialAudioFormatConfiguration[] = L"Windows.Media.Audio.SpatialAudioFormatConfiguration";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Audio.SpatialAudioFormatSubtype
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics2 interface starting with version 12.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Audio.ISpatialAudioFormatSubtypeStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatSubtype_DEFINED
#define RUNTIMECLASS_Windows_Media_Audio_SpatialAudioFormatSubtype_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Audio_SpatialAudioFormatSubtype[] = L"Windows.Media.Audio.SpatialAudioFormatSubtype";
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
#endif // __windows2Emedia2Eaudio_p_h__

#endif // __windows2Emedia2Eaudio_h__
