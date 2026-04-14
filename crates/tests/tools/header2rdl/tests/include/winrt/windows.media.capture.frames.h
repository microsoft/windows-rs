
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
#ifndef __windows2Emedia2Ecapture2Eframes_h__
#define __windows2Emedia2Ecapture2Eframes_h__
#ifndef __windows2Emedia2Ecapture2Eframes_p_h__
#define __windows2Emedia2Ecapture2Eframes_p_h__


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
#include "Windows.Graphics.DirectX.Direct3D11.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Media.h"
#include "Windows.Media.Capture.h"
#include "Windows.Media.Devices.h"
#include "Windows.Media.Devices.Core.h"
#include "Windows.Media.MediaProperties.h"
#include "Windows.Perception.Spatial.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.WindowManagement.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IAudioMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame ABI::Windows::Media::Capture::Frames::IAudioMediaFrame

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IBufferMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame ABI::Windows::Media::Capture::Frames::IBufferMediaFrame

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IDepthMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame ABI::Windows::Media::Capture::Frames::IDepthMediaFrame

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IDepthMediaFrame2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2 ABI::Windows::Media::Capture::Frames::IDepthMediaFrame2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IDepthMediaFrameFormat;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat ABI::Windows::Media::Capture::Frames::IDepthMediaFrameFormat

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IInfraredMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame ABI::Windows::Media::Capture::Frames::IInfraredMediaFrame

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameArrivedEventArgs;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs ABI::Windows::Media::Capture::Frames::IMediaFrameArrivedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameFormat;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat ABI::Windows::Media::Capture::Frames::IMediaFrameFormat

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameFormat2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2 ABI::Windows::Media::Capture::Frames::IMediaFrameFormat2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameReader;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader ABI::Windows::Media::Capture::Frames::IMediaFrameReader

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameReader2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2 ABI::Windows::Media::Capture::Frames::IMediaFrameReader2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameReference;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference ABI::Windows::Media::Capture::Frames::IMediaFrameReference

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameReference2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2 ABI::Windows::Media::Capture::Frames::IMediaFrameReference2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSource;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource ABI::Windows::Media::Capture::Frames::IMediaFrameSource

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceController;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController ABI::Windows::Media::Capture::Frames::IMediaFrameSourceController

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceController2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2 ABI::Windows::Media::Capture::Frames::IMediaFrameSourceController2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceController3;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3 ABI::Windows::Media::Capture::Frames::IMediaFrameSourceController3

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceGetPropertyResult;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGetPropertyResult

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceGroup;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceGroupStatics;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroupStatics

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceInfo;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceInfo2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2 ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceInfo3;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3 ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo3

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMediaFrameSourceInfo4;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4 ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo4

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMultiSourceMediaFrameArrivedEventArgs;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameArrivedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMultiSourceMediaFrameReader;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameReader

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMultiSourceMediaFrameReader2;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2 ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameReader2

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IMultiSourceMediaFrameReference;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameReference

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IVideoMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame ABI::Windows::Media::Capture::Frames::IVideoMediaFrame

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    interface IVideoMediaFrameFormat;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat ABI::Windows::Media::Capture::Frames::IVideoMediaFrameFormat

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameSourceGroup;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dc0c1f9a-b748-5cfa-9b42-a3a8fe37281a"))
IIterator<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0b71deb-76e8-5833-9623-2b1e1a8e1b72"))
IIterable<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d01148ae-cccd-56eb-b2b4-a7d2acce14ec"))
IVectorView<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a795889f-6d49-5687-aabe-f2fc6237fa1a"))
IAsyncOperation<__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup*> __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cff78a64-bd44-5638-af2f-540c23b322e7"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    typedef enum MediaFrameReaderStartStatus : int MediaFrameReaderStartStatus;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("db8e251a-adc6-5753-8784-c44b4d7c5b07"))
IAsyncOperation<enum ABI::Windows::Media::Capture::Frames::MediaFrameReaderStartStatus> : IAsyncOperation_impl<enum ABI::Windows::Media::Capture::Frames::MediaFrameReaderStartStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Capture.Frames.MediaFrameReaderStartStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::Capture::Frames::MediaFrameReaderStartStatus> __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9f49b2e5-2f68-5c58-8d8b-12176ff6ea50"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Capture::Frames::MediaFrameReaderStartStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::Capture::Frames::MediaFrameReaderStartStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Capture.Frames.MediaFrameReaderStartStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Capture::Frames::MediaFrameReaderStartStatus> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameSourceGetPropertyResult;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("80003979-4986-52a7-b227-ae6be4d2b5cd"))
IAsyncOperation<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyResult*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGetPropertyResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyResult*> __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d0577f0d-ce46-5c47-8f7c-4ae5626cc76d"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyResult*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGetPropertyResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f3256a87-b1cf-5943-b664-9f19367d2779"))
IAsyncOperation<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("adf10eeb-9fc5-553b-9164-294246992a2a"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Capture.Frames.MediaFrameSourceGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Capture::Frames::MediaFrameSourceGroup*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    typedef enum MediaFrameSourceSetPropertyStatus : int MediaFrameSourceSetPropertyStatus;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a1507c16-5f84-586e-8ca9-224f37e0e0de"))
IAsyncOperation<enum ABI::Windows::Media::Capture::Frames::MediaFrameSourceSetPropertyStatus> : IAsyncOperation_impl<enum ABI::Windows::Media::Capture::Frames::MediaFrameSourceSetPropertyStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::Capture::Frames::MediaFrameSourceSetPropertyStatus> __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f613663a-c685-5dc0-b133-60d94303a6e3"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Capture::Frames::MediaFrameSourceSetPropertyStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::Capture::Frames::MediaFrameSourceSetPropertyStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Capture::Frames::MediaFrameSourceSetPropertyStatus> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    typedef enum MultiSourceMediaFrameReaderStartStatus : int MultiSourceMediaFrameReaderStartStatus;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a242b952-76aa-54e5-a13b-a8707c1098e1"))
IAsyncOperation<enum ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReaderStartStatus> : IAsyncOperation_impl<enum ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReaderStartStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReaderStartStatus> __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_t;
#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("18eef24a-3332-5fee-a0f0-72ceed330645"))
IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReaderStartStatus> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReaderStartStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReaderStartStatus> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000


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
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameFormat;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE
#define DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("83a0cfaa-6546-5a63-8cd7-f62152a75d27"))
IIterator<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*, ABI::Windows::Media::Capture::Frames::IMediaFrameFormat*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Capture.Frames.MediaFrameFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*> __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_t;
#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE
#define DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1f029a27-1123-538a-9261-8a380e12bac6"))
IIterable<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*, ABI::Windows::Media::Capture::Frames::IMediaFrameFormat*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Capture.Frames.MediaFrameFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*> __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_t;
#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameSourceInfo;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE
#define DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("443c60d8-208e-5399-bc44-edc6fef02293"))
IIterator<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Capture.Frames.MediaFrameSourceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*> __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_t;
#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE
#define DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2e29c5b0-6aa9-50f2-91a4-5b67a5598f2e"))
IIterable<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Capture.Frames.MediaFrameSourceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*> __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_t;
#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                class MediaCaptureVideoProfileMediaDescription;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                interface IMediaCaptureVideoProfileMediaDescription;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription ABI::Windows::Media::Capture::IMediaCaptureVideoProfileMediaDescription

#endif // ____x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE
#define DEF___FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b3e8378f-710c-5126-a6c9-8f489f63e15e"))
IIterator<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*, ABI::Windows::Media::Capture::IMediaCaptureVideoProfileMediaDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*> __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_t;
#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE
#define DEF___FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0895e56d-fe1f-5364-ab67-c597d8970b89"))
IIterable<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*, ABI::Windows::Media::Capture::IMediaCaptureVideoProfileMediaDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*> __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_t;
#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bae2547d-3fae-55cd-b209-45c3b5b2f816"))
IVectorView<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*, ABI::Windows::Media::Capture::Frames::IMediaFrameFormat*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Capture.Frames.MediaFrameFormat>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Capture::Frames::MediaFrameFormat*> __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_t;
#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("dbdb7946-9b30-51d0-9c8c-c7105af690e0"))
IVectorView<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*, ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Capture.Frames.MediaFrameSourceInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Capture::Frames::MediaFrameSourceInfo*> __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_t;
#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("726c7c8c-789e-5fcb-b31f-f9d9d4a3ac42"))
IVectorView<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*, ABI::Windows::Media::Capture::IMediaCaptureVideoProfileMediaDescription*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Capture.MediaCaptureVideoProfileMediaDescription>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Capture::MediaCaptureVideoProfileMediaDescription*> __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_t;
#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_USE */

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
            namespace Capture {
                namespace Frames {
                    class MediaFrameReader;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameArrivedEventArgs;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3dd49cb-8d25-591a-80f7-8363d5c03ec9"))
ITypedEventHandler<ABI::Windows::Media::Capture::Frames::MediaFrameReader*, ABI::Windows::Media::Capture::Frames::MediaFrameArrivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameReader*, ABI::Windows::Media::Capture::Frames::IMediaFrameReader*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameArrivedEventArgs*, ABI::Windows::Media::Capture::Frames::IMediaFrameArrivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Capture.Frames.MediaFrameReader, Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Capture::Frames::MediaFrameReader*, ABI::Windows::Media::Capture::Frames::MediaFrameArrivedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameSource;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c7e61aa7-4716-5514-a913-ef1796b98dbd"))
ITypedEventHandler<ABI::Windows::Media::Capture::Frames::MediaFrameSource*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MediaFrameSource*, ABI::Windows::Media::Capture::Frames::IMediaFrameSource*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Capture.Frames.MediaFrameSource, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Capture::Frames::MediaFrameSource*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MultiSourceMediaFrameReader;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MultiSourceMediaFrameArrivedEventArgs;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("82b1ad4d-9887-56f4-9a9e-3ab18b02198c"))
ITypedEventHandler<ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReader*, ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameArrivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReader*, ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameReader*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameArrivedEventArgs*, ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameArrivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Capture.Frames.MultiSourceMediaFrameReader, Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameReader*, ABI::Windows::Media::Capture::Frames::MultiSourceMediaFrameArrivedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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
            namespace Capture {
                typedef enum MediaStreamType : int MediaStreamType;
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class AudioDeviceController;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class CameraIntrinsics;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface ICameraIntrinsics;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics ABI::Windows::Media::Devices::Core::ICameraIntrinsics

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    class DepthCorrelatedCoordinateMapper;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                namespace Core {
                    interface IDepthCorrelatedCoordinateMapper;
                } /* Core */
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper ABI::Windows::Media::Devices::Core::IDepthCorrelatedCoordinateMapper

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Devices {
                class VideoDeviceController;
            } /* Devices */
        } /* Media */
    } /* Windows */
} /* ABI */

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
            class VideoFrame;
        } /* Media */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                class SpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Perception {
            namespace Spatial {
                interface ISpatialCoordinateSystem;
            } /* Spatial */
        } /* Perception */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

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
        namespace UI {
            namespace WindowManagement {
                class DisplayRegion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace WindowManagement {
                interface IDisplayRegion;
            } /* WindowManagement */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion ABI::Windows::UI::WindowManagement::IDisplayRegion

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    typedef enum MediaFrameReaderAcquisitionMode : int MediaFrameReaderAcquisitionMode;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    typedef enum MediaFrameSourceGetPropertyStatus : int MediaFrameSourceGetPropertyStatus;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    typedef enum MediaFrameSourceKind : int MediaFrameSourceKind;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class AudioMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class BufferMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class DepthMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class DepthMediaFrameFormat;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class InfraredMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameReference;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MediaFrameSourceController;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class MultiSourceMediaFrameReference;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class VideoMediaFrame;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    class VideoMediaFrameFormat;
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameReaderAcquisitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    enum MediaFrameReaderAcquisitionMode : int
                    {
                        MediaFrameReaderAcquisitionMode_Realtime = 0,
                        MediaFrameReaderAcquisitionMode_Buffered = 1,
                    };
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameReaderStartStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    enum MediaFrameReaderStartStatus : int
                    {
                        MediaFrameReaderStartStatus_Success = 0,
                        MediaFrameReaderStartStatus_UnknownFailure = 1,
                        MediaFrameReaderStartStatus_DeviceNotAvailable = 2,
                        MediaFrameReaderStartStatus_OutputFormatNotSupported = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        MediaFrameReaderStartStatus_ExclusiveControlNotAvailable = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    };
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    enum MediaFrameSourceGetPropertyStatus : int
                    {
                        MediaFrameSourceGetPropertyStatus_Success = 0,
                        MediaFrameSourceGetPropertyStatus_UnknownFailure = 1,
                        MediaFrameSourceGetPropertyStatus_NotSupported = 2,
                        MediaFrameSourceGetPropertyStatus_DeviceNotAvailable = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        MediaFrameSourceGetPropertyStatus_MaxPropertyValueSizeTooSmall = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                        MediaFrameSourceGetPropertyStatus_MaxPropertyValueSizeRequired = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    };
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameSourceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    enum MediaFrameSourceKind : int
                    {
                        MediaFrameSourceKind_Custom = 0,
                        MediaFrameSourceKind_Color = 1,
                        MediaFrameSourceKind_Infrared = 2,
                        MediaFrameSourceKind_Depth = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        MediaFrameSourceKind_Audio = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                        MediaFrameSourceKind_Image = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                        MediaFrameSourceKind_Metadata = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
                    };
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    enum MediaFrameSourceSetPropertyStatus : int
                    {
                        MediaFrameSourceSetPropertyStatus_Success = 0,
                        MediaFrameSourceSetPropertyStatus_UnknownFailure = 1,
                        MediaFrameSourceSetPropertyStatus_NotSupported = 2,
                        MediaFrameSourceSetPropertyStatus_InvalidValue = 3,
                        MediaFrameSourceSetPropertyStatus_DeviceNotAvailable = 4,
                        MediaFrameSourceSetPropertyStatus_NotInControl = 5,
                    };
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    enum MultiSourceMediaFrameReaderStartStatus : int
                    {
                        MultiSourceMediaFrameReaderStartStatus_Success = 0,
                        MultiSourceMediaFrameReaderStartStatus_NotSupported = 1,
                        MultiSourceMediaFrameReaderStartStatus_InsufficientResources = 2,
                        MultiSourceMediaFrameReaderStartStatus_DeviceNotAvailable = 3,
                        MultiSourceMediaFrameReaderStartStatus_UnknownFailure = 4,
                    };
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IAudioMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.AudioMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IAudioMediaFrame[] = L"Windows.Media.Capture.Frames.IAudioMediaFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("a3a9feff-8021-441b-9a46-e7f0137b7981")
                    IAudioMediaFrame : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FrameReference(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioEncodingProperties(
                            ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetAudioFrame(
                            ABI::Windows::Media::IAudioFrame** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAudioMediaFrame = __uuidof(IAudioMediaFrame);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IBufferMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.BufferMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IBufferMediaFrame[] = L"Windows.Media.Capture.Frames.IBufferMediaFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("b5b153c7-9b84-4062-b79c-a365b2596854")
                    IBufferMediaFrame : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FrameReference(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Buffer(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBufferMediaFrame = __uuidof(IBufferMediaFrame);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IDepthMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.DepthMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IDepthMediaFrame[] = L"Windows.Media.Capture.Frames.IDepthMediaFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("47135e4f-8549-45c0-925b-80d35efdb10a")
                    IDepthMediaFrame : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FrameReference(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IVideoMediaFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DepthFormat(
                            ABI::Windows::Media::Capture::Frames::IDepthMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryCreateCoordinateMapper(
                            ABI::Windows::Media::Devices::Core::ICameraIntrinsics* cameraIntrinsics,
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem* coordinateSystem,
                            ABI::Windows::Media::Devices::Core::IDepthCorrelatedCoordinateMapper** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDepthMediaFrame = __uuidof(IDepthMediaFrame);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IDepthMediaFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.DepthMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IDepthMediaFrame2[] = L"Windows.Media.Capture.Frames.IDepthMediaFrame2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("6cca473d-c4a4-4176-b0cd-33eae3b35aa3")
                    IDepthMediaFrame2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MaxReliableDepth(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MinReliableDepth(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDepthMediaFrame2 = __uuidof(IDepthMediaFrame2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IDepthMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.DepthMediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IDepthMediaFrameFormat[] = L"Windows.Media.Capture.Frames.IDepthMediaFrameFormat";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("c312cf40-d729-453e-8780-2e04f140d28e")
                    IDepthMediaFrameFormat : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_VideoFormat(
                            ABI::Windows::Media::Capture::Frames::IVideoMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DepthScaleInMeters(
                            DOUBLE* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IDepthMediaFrameFormat = __uuidof(IDepthMediaFrameFormat);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IInfraredMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.InfraredMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IInfraredMediaFrame[] = L"Windows.Media.Capture.Frames.IInfraredMediaFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("3fd13503-004b-4f0e-91ac-465299b41658")
                    IInfraredMediaFrame : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FrameReference(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IVideoMediaFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsIlluminated(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IInfraredMediaFrame = __uuidof(IInfraredMediaFrame);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("0b430add-a490-4435-ada1-9affd55239f7")
                    IMediaFrameArrivedEventArgs : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameArrivedEventArgs = __uuidof(IMediaFrameArrivedEventArgs);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameFormat[] = L"Windows.Media.Capture.Frames.IMediaFrameFormat";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("71902b4e-b279-4a97-a9db-bd5a2fb78f39")
                    IMediaFrameFormat : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MajorType(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Subtype(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FrameRate(
                            ABI::Windows::Media::MediaProperties::IMediaRatio** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoFormat(
                            ABI::Windows::Media::Capture::Frames::IVideoMediaFrameFormat** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameFormat = __uuidof(IMediaFrameFormat);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameFormat2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameFormat2[] = L"Windows.Media.Capture.Frames.IMediaFrameFormat2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("63856340-5e87-4c10-86d1-6df097a6c6a8")
                    IMediaFrameFormat2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AudioEncodingProperties(
                            ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameFormat2 = __uuidof(IMediaFrameFormat2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReader[] = L"Windows.Media.Capture.Frames.IMediaFrameReader";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("e4c94395-2028-48ed-90b0-d1c1b162e24c")
                    IMediaFrameReader : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_FrameArrived(
                            __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FrameArrived(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireLatestFrame(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StartAsync(
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAsync(
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameReader = __uuidof(IMediaFrameReader);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReader2[] = L"Windows.Media.Capture.Frames.IMediaFrameReader2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("871127b3-8531-4050-87cc-a13733cf3e9b")
                    IMediaFrameReader2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_AcquisitionMode(
                            ABI::Windows::Media::Capture::Frames::MediaFrameReaderAcquisitionMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AcquisitionMode(
                            ABI::Windows::Media::Capture::Frames::MediaFrameReaderAcquisitionMode* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameReader2 = __uuidof(IMediaFrameReader2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReference
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReference[] = L"Windows.Media.Capture.Frames.IMediaFrameReference";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("f6b88641-f0dc-4044-8dc9-961cedd05bad")
                    IMediaFrameReference : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SourceKind(
                            ABI::Windows::Media::Capture::Frames::MediaFrameSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Format(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SystemRelativeTime(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Duration(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BufferMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IBufferMediaFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IVideoMediaFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameReference = __uuidof(IMediaFrameReference);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReference2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReference2[] = L"Windows.Media.Capture.Frames.IMediaFrameReference2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("ddbc3ecc-d5b2-49ef-836a-947d989b80c1")
                    IMediaFrameReference2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AudioMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IAudioMediaFrame** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameReference2 = __uuidof(IMediaFrameReference2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSource[] = L"Windows.Media.Capture.Frames.IMediaFrameSource";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("d6782953-90db-46a8-8add-2aa884a8d253")
                    IMediaFrameSource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Info(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameSourceInfo** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Controller(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameSourceController** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedFormats(
                            __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentFormat(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetFormatAsync(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameFormat* format,
                            ABI::Windows::Foundation::IAsyncAction** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_FormatChanged(
                            __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FormatChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryGetCameraIntrinsics(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameFormat* format,
                            ABI::Windows::Media::Devices::Core::ICameraIntrinsics** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSource = __uuidof(IMediaFrameSource);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceController[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceController";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("6d076635-316d-4b8f-b7b6-eeb04a8c6525")
                    IMediaFrameSourceController : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetPropertyAsync(
                            HSTRING propertyId,
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyAsync(
                            HSTRING propertyId,
                            IInspectable* propertyValue,
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoDeviceController(
                            ABI::Windows::Media::Devices::IVideoDeviceController** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceController = __uuidof(IMediaFrameSourceController);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceController2[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceController2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("efc49fd4-fcf2-4a03-b4e4-ac9628739bee")
                    IMediaFrameSourceController2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetPropertyByExtendedIdAsync(
                            UINT32 extendedPropertyIdLength,
                            BYTE* extendedPropertyId,
                            __FIReference_1_UINT32* maxPropertyValueSize,
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE SetPropertyByExtendedIdAsync(
                            UINT32 extendedPropertyIdLength,
                            BYTE* extendedPropertyId,
                            UINT32 propertyValueLength,
                            BYTE* propertyValue,
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceController2 = __uuidof(IMediaFrameSourceController2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceController3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceController3[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceController3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("1f0cf815-2464-4651-b1e8-4a82dbdb54de")
                    IMediaFrameSourceController3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AudioDeviceController(
                            ABI::Windows::Media::Devices::IAudioDeviceController** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceController3 = __uuidof(IMediaFrameSourceController3);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceGetPropertyResult[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("088616c2-3a64-4bd5-bd2b-e7c898d2f37a")
                    IMediaFrameSourceGetPropertyResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Media::Capture::Frames::MediaFrameSourceGetPropertyStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Value(
                            IInspectable** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceGetPropertyResult = __uuidof(IMediaFrameSourceGetPropertyResult);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceGroup[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceGroup";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("7f605b87-4832-4b5f-ae3d-412faab37d34")
                    IMediaFrameSourceGroup : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceInfos(
                            __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceGroup = __uuidof(IMediaFrameSourceGroup);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceGroupStatics[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("1c48bfc5-436f-4508-94cf-d5d8b7326445")
                    IMediaFrameSourceGroupStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE FindAllAsync(
                            __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE FromIdAsync(
                            HSTRING id,
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeviceSelector(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceGroupStatics = __uuidof(IMediaFrameSourceGroupStatics);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("87bdc9cd-4601-408f-91cf-038318cd0af3")
                    IMediaFrameSourceInfo : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MediaStreamType(
                            ABI::Windows::Media::Capture::MediaStreamType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceKind(
                            ABI::Windows::Media::Capture::Frames::MediaFrameSourceKind* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SourceGroup(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameSourceGroup** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DeviceInformation(
                            ABI::Windows::Devices::Enumeration::IDeviceInformation** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Properties(
                            __FIMapView_2_GUID_IInspectable** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CoordinateSystem(
                            ABI::Windows::Perception::Spatial::ISpatialCoordinateSystem** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceInfo = __uuidof(IMediaFrameSourceInfo);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo2[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("195a7855-6457-42c6-a769-19b65bd32e6e")
                    IMediaFrameSourceInfo2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ProfileId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoProfileMediaDescription(
                            __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceInfo2 = __uuidof(IMediaFrameSourceInfo2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo3[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("ca824ab6-66ea-5885-a2b6-26c0eeec3c7b")
                    IMediaFrameSourceInfo3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetRelativePanel(
                            ABI::Windows::UI::WindowManagement::IDisplayRegion* displayRegion,
                            ABI::Windows::Devices::Enumeration::Panel* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceInfo3 = __uuidof(IMediaFrameSourceInfo3);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo4[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo4";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("4817d721-85eb-470c-8f37-43ca5498e41d")
                    IMediaFrameSourceInfo4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsShareable(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMediaFrameSourceInfo4 = __uuidof(IMediaFrameSourceInfo4);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("63115e01-cf51-48fd-aab0-6d693eb48127")
                    IMultiSourceMediaFrameArrivedEventArgs : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IMultiSourceMediaFrameArrivedEventArgs = __uuidof(IMultiSourceMediaFrameArrivedEventArgs);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameReader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameReader[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("8d144402-f763-488d-98f2-b437bcf075e7")
                    IMultiSourceMediaFrameReader : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_FrameArrived(
                            __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FrameArrived(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireLatestFrame(
                            ABI::Windows::Media::Capture::Frames::IMultiSourceMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StartAsync(
                            __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAsync(
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMultiSourceMediaFrameReader = __uuidof(IMultiSourceMediaFrameReader);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameReader2[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("ef5c8abd-fc5c-4c6b-9d81-3cb9cc637c26")
                    IMultiSourceMediaFrameReader2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE put_AcquisitionMode(
                            ABI::Windows::Media::Capture::Frames::MediaFrameReaderAcquisitionMode value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AcquisitionMode(
                            ABI::Windows::Media::Capture::Frames::MediaFrameReaderAcquisitionMode* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMultiSourceMediaFrameReader2 = __uuidof(IMultiSourceMediaFrameReader2);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameReference
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameReference[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("21964b1a-7fe2-44d6-92e5-298e6d2810e9")
                    IMultiSourceMediaFrameReference : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE TryGetFrameReferenceBySourceId(
                            HSTRING sourceId,
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IMultiSourceMediaFrameReference = __uuidof(IMultiSourceMediaFrameReference);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IVideoMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.VideoMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IVideoMediaFrame[] = L"Windows.Media.Capture.Frames.IVideoMediaFrame";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("00dd4ccb-32bd-4fe1-a013-7cc13cf5dbcf")
                    IVideoMediaFrame : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FrameReference(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameReference** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoFormat(
                            ABI::Windows::Media::Capture::Frames::IVideoMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SoftwareBitmap(
                            ABI::Windows::Graphics::Imaging::ISoftwareBitmap** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Direct3DSurface(
                            ABI::Windows::Graphics::DirectX::Direct3D11::IDirect3DSurface** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CameraIntrinsics(
                            ABI::Windows::Media::Devices::Core::ICameraIntrinsics** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InfraredMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IInfraredMediaFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DepthMediaFrame(
                            ABI::Windows::Media::Capture::Frames::IDepthMediaFrame** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetVideoFrame(
                            ABI::Windows::Media::IVideoFrame** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVideoMediaFrame = __uuidof(IVideoMediaFrame);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IVideoMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.VideoMediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IVideoMediaFrameFormat[] = L"Windows.Media.Capture.Frames.IVideoMediaFrameFormat";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Capture {
                namespace Frames {
                    MIDL_INTERFACE("46027fc0-d71b-45c7-8f14-6d9a0ae604e4")
                    IVideoMediaFrameFormat : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MediaFrameFormat(
                            ABI::Windows::Media::Capture::Frames::IMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DepthFormat(
                            ABI::Windows::Media::Capture::Frames::IDepthMediaFrameFormat** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Width(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Height(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IVideoMediaFrameFormat = __uuidof(IVideoMediaFrameFormat);
                } /* Frames */
            } /* Capture */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.AudioMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IAudioMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_AudioMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_AudioMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_AudioMediaFrame[] = L"Windows.Media.Capture.Frames.AudioMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Capture.Frames.BufferMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IBufferMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_BufferMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_BufferMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_BufferMediaFrame[] = L"Windows.Media.Capture.Frames.BufferMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.DepthMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IDepthMediaFrame ** Default Interface **
 *    Windows.Media.Capture.Frames.IDepthMediaFrame2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_DepthMediaFrame[] = L"Windows.Media.Capture.Frames.DepthMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.DepthMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IDepthMediaFrameFormat ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrameFormat_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrameFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_DepthMediaFrameFormat[] = L"Windows.Media.Capture.Frames.DepthMediaFrameFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.InfraredMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IInfraredMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_InfraredMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_InfraredMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_InfraredMediaFrame[] = L"Windows.Media.Capture.Frames.InfraredMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameArrivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameArrivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameFormat ** Default Interface **
 *    Windows.Media.Capture.Frames.IMediaFrameFormat2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameFormat_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameFormat[] = L"Windows.Media.Capture.Frames.MediaFrameFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Capture.Frames.IMediaFrameReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReader_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameReader[] = L"Windows.Media.Capture.Frames.MediaFrameReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameReference ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Capture.Frames.IMediaFrameReference2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReference_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameReference[] = L"Windows.Media.Capture.Frames.MediaFrameReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSource[] = L"Windows.Media.Capture.Frames.MediaFrameSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceController ** Default Interface **
 *    Windows.Media.Capture.Frames.IMediaFrameSourceController2
 *    Windows.Media.Capture.Frames.IMediaFrameSourceController3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceController[] = L"Windows.Media.Capture.Frames.MediaFrameSourceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGetPropertyResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGetPropertyResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceGetPropertyResult[] = L"Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGroup_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceGroup[] = L"Windows.Media.Capture.Frames.MediaFrameSourceGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo ** Default Interface **
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo2
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo3
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceInfo_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceInfo[] = L"Windows.Media.Capture.Frames.MediaFrameSourceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameArrivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameArrivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MultiSourceMediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Capture.Frames.MultiSourceMediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReader_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MultiSourceMediaFrameReader[] = L"Windows.Media.Capture.Frames.MultiSourceMediaFrameReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Capture.Frames.MultiSourceMediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReference_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MultiSourceMediaFrameReference[] = L"Windows.Media.Capture.Frames.MultiSourceMediaFrameReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Capture.Frames.VideoMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IVideoMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_VideoMediaFrame[] = L"Windows.Media.Capture.Frames.VideoMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.VideoMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IVideoMediaFrameFormat ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrameFormat_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrameFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_VideoMediaFrameFormat[] = L"Windows.Media.Capture.Frames.VideoMediaFrameFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2 __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderStartStatus __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderStartStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderStartStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatusVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceSetPropertyStatus __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceSetPropertyStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceSetPropertyStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatusVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

typedef enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMultiSourceMediaFrameReaderStartStatus __x_ABI_CWindows_CMedia_CCapture_CFrames_CMultiSourceMediaFrameReaderStartStatus;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMultiSourceMediaFrameReaderStartStatus* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatusVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatusVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat;

typedef struct __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl;

interface __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat;

typedef struct __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl;

interface __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo;

typedef struct __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl;

interface __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo;

typedef struct __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        __FIIterator_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl;

interface __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription;

#endif // ____x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription;

typedef struct __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl;

interface __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription;

typedef struct __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        __FIIterator_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl;

interface __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat;

typedef struct __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl;

interface __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo;

typedef struct __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl;

interface __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription;

typedef struct __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCapture_CIMediaCaptureVideoProfileMediaDescription** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl;

interface __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescriptionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* sender,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* sender,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation;

#endif // ____x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel __x_ABI_CWindows_CDevices_CEnumeration_CPanel;

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

#ifndef ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface;

#endif // ____x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
#define ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap;

#endif // ____x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIAudioFrame __x_ABI_CWindows_CMedia_CIAudioFrame;

#endif // ____x_ABI_CWindows_CMedia_CIAudioFrame_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType;

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController;

#endif // ____x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CIVideoFrame __x_ABI_CWindows_CMedia_CIVideoFrame;

#endif // ____x_ABI_CWindows_CMedia_CIVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
#define ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem;

#endif // ____x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion;

#endif // ____x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceGetPropertyStatus __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceGetPropertyStatus;

typedef enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceKind __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceKind;

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameReaderAcquisitionMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode
{
    MediaFrameReaderAcquisitionMode_Realtime = 0,
    MediaFrameReaderAcquisitionMode_Buffered = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameReaderStartStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderStartStatus
{
    MediaFrameReaderStartStatus_Success = 0,
    MediaFrameReaderStartStatus_UnknownFailure = 1,
    MediaFrameReaderStartStatus_DeviceNotAvailable = 2,
    MediaFrameReaderStartStatus_OutputFormatNotSupported = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    MediaFrameReaderStartStatus_ExclusiveControlNotAvailable = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceGetPropertyStatus
{
    MediaFrameSourceGetPropertyStatus_Success = 0,
    MediaFrameSourceGetPropertyStatus_UnknownFailure = 1,
    MediaFrameSourceGetPropertyStatus_NotSupported = 2,
    MediaFrameSourceGetPropertyStatus_DeviceNotAvailable = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    MediaFrameSourceGetPropertyStatus_MaxPropertyValueSizeTooSmall = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    MediaFrameSourceGetPropertyStatus_MaxPropertyValueSizeRequired = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameSourceKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceKind
{
    MediaFrameSourceKind_Custom = 0,
    MediaFrameSourceKind_Color = 1,
    MediaFrameSourceKind_Infrared = 2,
    MediaFrameSourceKind_Depth = 3,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    MediaFrameSourceKind_Audio = 4,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    MediaFrameSourceKind_Image = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
    MediaFrameSourceKind_Metadata = 6,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xc0000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MediaFrameSourceSetPropertyStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceSetPropertyStatus
{
    MediaFrameSourceSetPropertyStatus_Success = 0,
    MediaFrameSourceSetPropertyStatus_UnknownFailure = 1,
    MediaFrameSourceSetPropertyStatus_NotSupported = 2,
    MediaFrameSourceSetPropertyStatus_InvalidValue = 3,
    MediaFrameSourceSetPropertyStatus_DeviceNotAvailable = 4,
    MediaFrameSourceSetPropertyStatus_NotInControl = 5,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.Capture.Frames.MultiSourceMediaFrameReaderStartStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMultiSourceMediaFrameReaderStartStatus
{
    MultiSourceMediaFrameReaderStartStatus_Success = 0,
    MultiSourceMediaFrameReaderStartStatus_NotSupported = 1,
    MultiSourceMediaFrameReaderStartStatus_InsufficientResources = 2,
    MultiSourceMediaFrameReaderStartStatus_DeviceNotAvailable = 3,
    MultiSourceMediaFrameReaderStartStatus_UnknownFailure = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IAudioMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.AudioMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IAudioMediaFrame[] = L"Windows.Media.Capture.Frames.IAudioMediaFrame";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FrameReference)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* get_AudioEncodingProperties)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* GetAudioFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame* This,
        __x_ABI_CWindows_CMedia_CIAudioFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrameVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_get_FrameReference(This, value) \
    ((This)->lpVtbl->get_FrameReference(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_get_AudioEncodingProperties(This, value) \
    ((This)->lpVtbl->get_AudioEncodingProperties(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_GetAudioFrame(This, value) \
    ((This)->lpVtbl->GetAudioFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IBufferMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.BufferMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IBufferMediaFrame[] = L"Windows.Media.Capture.Frames.IBufferMediaFrame";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FrameReference)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* get_Buffer)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrameVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_get_FrameReference(This, value) \
    ((This)->lpVtbl->get_FrameReference(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_get_Buffer(This, value) \
    ((This)->lpVtbl->get_Buffer(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IDepthMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.DepthMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IDepthMediaFrame[] = L"Windows.Media.Capture.Frames.IDepthMediaFrame";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FrameReference)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_DepthFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* TryCreateCoordinateMapper)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics* cameraIntrinsics,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem* coordinateSystem,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CIDepthCorrelatedCoordinateMapper** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_get_FrameReference(This, value) \
    ((This)->lpVtbl->get_FrameReference(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_get_VideoMediaFrame(This, value) \
    ((This)->lpVtbl->get_VideoMediaFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_get_DepthFormat(This, value) \
    ((This)->lpVtbl->get_DepthFormat(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_TryCreateCoordinateMapper(This, cameraIntrinsics, coordinateSystem, value) \
    ((This)->lpVtbl->TryCreateCoordinateMapper(This, cameraIntrinsics, coordinateSystem, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IDepthMediaFrame2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.DepthMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IDepthMediaFrame2[] = L"Windows.Media.Capture.Frames.IDepthMediaFrame2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MaxReliableDepth)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MinReliableDepth)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_get_MaxReliableDepth(This, value) \
    ((This)->lpVtbl->get_MaxReliableDepth(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_get_MinReliableDepth(This, value) \
    ((This)->lpVtbl->get_MinReliableDepth(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IDepthMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.DepthMediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IDepthMediaFrameFormat[] = L"Windows.Media.Capture.Frames.IDepthMediaFrameFormat";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_VideoFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_DepthScaleInMeters)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormatVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_get_VideoFormat(This, value) \
    ((This)->lpVtbl->get_VideoFormat(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_get_DepthScaleInMeters(This, value) \
    ((This)->lpVtbl->get_DepthScaleInMeters(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IInfraredMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.InfraredMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IInfraredMediaFrame[] = L"Windows.Media.Capture.Frames.IInfraredMediaFrame";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FrameReference)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_IsIlluminated)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrameVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_get_FrameReference(This, value) \
    ((This)->lpVtbl->get_FrameReference(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_get_VideoMediaFrame(This, value) \
    ((This)->lpVtbl->get_VideoMediaFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_get_IsIlluminated(This, value) \
    ((This)->lpVtbl->get_IsIlluminated(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameArrivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameFormat[] = L"Windows.Media.Capture.Frames.IMediaFrameFormat";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MajorType)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Subtype)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FrameRate)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        __FIMapView_2_GUID_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormatVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_get_MajorType(This, value) \
    ((This)->lpVtbl->get_MajorType(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_get_Subtype(This, value) \
    ((This)->lpVtbl->get_Subtype(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_get_FrameRate(This, value) \
    ((This)->lpVtbl->get_FrameRate(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_get_VideoFormat(This, value) \
    ((This)->lpVtbl->get_VideoFormat(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameFormat2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameFormat2[] = L"Windows.Media.Capture.Frames.IMediaFrameFormat2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioEncodingProperties)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_get_AudioEncodingProperties(This, value) \
    ((This)->lpVtbl->get_AudioEncodingProperties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReader[] = L"Windows.Media.Capture.Frames.IMediaFrameReader";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_FrameArrived)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMediaFrameArrivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameArrived)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* TryAcquireLatestFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameReaderStartStatus** operation);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReaderVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_add_FrameArrived(This, handler, token) \
    ((This)->lpVtbl->add_FrameArrived(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_remove_FrameArrived(This, token) \
    ((This)->lpVtbl->remove_FrameArrived(This, token))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_TryAcquireLatestFrame(This, value) \
    ((This)->lpVtbl->TryAcquireLatestFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_StopAsync(This, action) \
    ((This)->lpVtbl->StopAsync(This, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReader2[] = L"Windows.Media.Capture.Frames.IMediaFrameReader2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_AcquisitionMode)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode value);
    HRESULT (STDMETHODCALLTYPE* get_AcquisitionMode)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_put_AcquisitionMode(This, value) \
    ((This)->lpVtbl->put_AcquisitionMode(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_get_AcquisitionMode(This, value) \
    ((This)->lpVtbl->get_AcquisitionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReference
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReference[] = L"Windows.Media.Capture.Frames.IMediaFrameReference";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceKind)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* get_Format)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_SystemRelativeTime)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_Duration)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        __FIMapView_2_GUID_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_BufferMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIBufferMediaFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReferenceVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_SourceKind(This, value) \
    ((This)->lpVtbl->get_SourceKind(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_Format(This, value) \
    ((This)->lpVtbl->get_Format(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_SystemRelativeTime(This, value) \
    ((This)->lpVtbl->get_SystemRelativeTime(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_Duration(This, value) \
    ((This)->lpVtbl->get_Duration(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_BufferMediaFrame(This, value) \
    ((This)->lpVtbl->get_BufferMediaFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_VideoMediaFrame(This, value) \
    ((This)->lpVtbl->get_VideoMediaFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameReference2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameReference
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameReference2[] = L"Windows.Media.Capture.Frames.IMediaFrameReference2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIAudioMediaFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_get_AudioMediaFrame(This, value) \
    ((This)->lpVtbl->get_AudioMediaFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSource[] = L"Windows.Media.Capture.Frames.IMediaFrameSource";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Info)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_Controller)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedFormats)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* SetFormatAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* format,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** value);
    HRESULT (STDMETHODCALLTYPE* add_FormatChanged)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMediaFrameSource_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FormatChanged)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* TryGetCameraIntrinsics)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat* format,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_get_Info(This, value) \
    ((This)->lpVtbl->get_Info(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_get_Controller(This, value) \
    ((This)->lpVtbl->get_Controller(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_get_SupportedFormats(This, value) \
    ((This)->lpVtbl->get_SupportedFormats(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_get_CurrentFormat(This, value) \
    ((This)->lpVtbl->get_CurrentFormat(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_SetFormatAsync(This, format, value) \
    ((This)->lpVtbl->SetFormatAsync(This, format, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_add_FormatChanged(This, handler, token) \
    ((This)->lpVtbl->add_FormatChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_remove_FormatChanged(This, token) \
    ((This)->lpVtbl->remove_FormatChanged(This, token))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_TryGetCameraIntrinsics(This, format, value) \
    ((This)->lpVtbl->TryGetCameraIntrinsics(This, format, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceController[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceController";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceControllerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPropertyAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        HSTRING propertyId,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult** value);
    HRESULT (STDMETHODCALLTYPE* SetPropertyAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        HSTRING propertyId,
        IInspectable* propertyValue,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoDeviceController)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController* This,
        __x_ABI_CWindows_CMedia_CDevices_CIVideoDeviceController** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceControllerVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceControllerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_GetPropertyAsync(This, propertyId, value) \
    ((This)->lpVtbl->GetPropertyAsync(This, propertyId, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_SetPropertyAsync(This, propertyId, propertyValue, value) \
    ((This)->lpVtbl->SetPropertyAsync(This, propertyId, propertyValue, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_get_VideoDeviceController(This, value) \
    ((This)->lpVtbl->get_VideoDeviceController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceController2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceController2[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceController2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetPropertyByExtendedIdAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This,
        UINT32 extendedPropertyIdLength,
        BYTE* extendedPropertyId,
        __FIReference_1_UINT32* maxPropertyValueSize,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGetPropertyResult** operation);
    HRESULT (STDMETHODCALLTYPE* SetPropertyByExtendedIdAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2* This,
        UINT32 extendedPropertyIdLength,
        BYTE* extendedPropertyId,
        UINT32 propertyValueLength,
        BYTE* propertyValue,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceSetPropertyStatus** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_GetPropertyByExtendedIdAsync(This, extendedPropertyIdLength, extendedPropertyId, maxPropertyValueSize, operation) \
    ((This)->lpVtbl->GetPropertyByExtendedIdAsync(This, extendedPropertyIdLength, extendedPropertyId, maxPropertyValueSize, operation))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_SetPropertyByExtendedIdAsync(This, extendedPropertyIdLength, extendedPropertyId, propertyValueLength, propertyValue, operation) \
    ((This)->lpVtbl->SetPropertyByExtendedIdAsync(This, extendedPropertyIdLength, extendedPropertyId, propertyValueLength, propertyValue, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceController3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceController3[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceController3";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AudioDeviceController)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3* This,
        __x_ABI_CWindows_CMedia_CDevices_CIAudioDeviceController** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_get_AudioDeviceController(This, value) \
    ((This)->lpVtbl->get_AudioDeviceController(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceController3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceGetPropertyResult[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceGetPropertyStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult* This,
        IInspectable** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResultVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_get_Value(This, value) \
    ((This)->lpVtbl->get_Value(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGetPropertyResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceGroup[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceGroup";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceInfos)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup* This,
        __FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_get_SourceInfos(This, value) \
    ((This)->lpVtbl->get_SourceInfos(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceGroupStatics[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindAllAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** value);
    HRESULT (STDMETHODCALLTYPE* FromIdAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        HSTRING id,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMediaFrameSourceGroup** value);
    HRESULT (STDMETHODCALLTYPE* GetDeviceSelector)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FindAllAsync(This, value) \
    ((This)->lpVtbl->FindAllAsync(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_FromIdAsync(This, id, value) \
    ((This)->lpVtbl->FromIdAsync(This, id, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_GetDeviceSelector(This, value) \
    ((This)->lpVtbl->GetDeviceSelector(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroupStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaStreamType)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CMediaStreamType* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceKind)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameSourceKind* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceGroup)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceGroup** value);
    HRESULT (STDMETHODCALLTYPE* get_DeviceInformation)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        __x_ABI_CWindows_CDevices_CEnumeration_CIDeviceInformation** value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        __FIMapView_2_GUID_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_CoordinateSystem)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo* This,
        __x_ABI_CWindows_CPerception_CSpatial_CISpatialCoordinateSystem** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfoVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_MediaStreamType(This, value) \
    ((This)->lpVtbl->get_MediaStreamType(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_SourceKind(This, value) \
    ((This)->lpVtbl->get_SourceKind(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_SourceGroup(This, value) \
    ((This)->lpVtbl->get_SourceGroup(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_DeviceInformation(This, value) \
    ((This)->lpVtbl->get_DeviceInformation(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_get_CoordinateSystem(This, value) \
    ((This)->lpVtbl->get_CoordinateSystem(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo2[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProfileId)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VideoProfileMediaDescription)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2* This,
        __FIVectorView_1_Windows__CMedia__CCapture__CMediaCaptureVideoProfileMediaDescription** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_get_ProfileId(This, value) \
    ((This)->lpVtbl->get_ProfileId(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_get_VideoProfileMediaDescription(This, value) \
    ((This)->lpVtbl->get_VideoProfileMediaDescription(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo3[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo3";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetRelativePanel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3* This,
        __x_ABI_CWindows_CUI_CWindowManagement_CIDisplayRegion* displayRegion,
        enum __x_ABI_CWindows_CDevices_CEnumeration_CPanel* result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_GetRelativePanel(This, displayRegion, result) \
    ((This)->lpVtbl->GetRelativePanel(This, displayRegion, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMediaFrameSourceInfo4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMediaFrameSourceInfo4[] = L"Windows.Media.Capture.Frames.IMediaFrameSourceInfo4";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsShareable)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_get_IsShareable(This, value) \
    ((This)->lpVtbl->get_IsShareable(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameSourceInfo4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameArrivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameReader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameReader[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_FrameArrived)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        __FITypedEventHandler_2_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReader_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameArrivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameArrived)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* TryAcquireLatestFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        __FIAsyncOperation_1_Windows__CMedia__CCapture__CFrames__CMultiSourceMediaFrameReaderStartStatus** operation);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReaderVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_add_FrameArrived(This, handler, token) \
    ((This)->lpVtbl->add_FrameArrived(This, handler, token))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_remove_FrameArrived(This, token) \
    ((This)->lpVtbl->remove_FrameArrived(This, token))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_TryAcquireLatestFrame(This, value) \
    ((This)->lpVtbl->TryAcquireLatestFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_StopAsync(This, action) \
    ((This)->lpVtbl->StopAsync(This, action))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameReader2[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_AcquisitionMode)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode value);
    HRESULT (STDMETHODCALLTYPE* get_AcquisitionMode)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2* This,
        enum __x_ABI_CWindows_CMedia_CCapture_CFrames_CMediaFrameReaderAcquisitionMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2Vtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_put_AcquisitionMode(This, value) \
    ((This)->lpVtbl->put_AcquisitionMode(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_get_AcquisitionMode(This, value) \
    ((This)->lpVtbl->get_AcquisitionMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.MultiSourceMediaFrameReference
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Foundation.IClosable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IMultiSourceMediaFrameReference[] = L"Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReferenceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryGetFrameReferenceBySourceId)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference* This,
        HSTRING sourceId,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReferenceVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReferenceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_TryGetFrameReferenceBySourceId(This, sourceId, value) \
    ((This)->lpVtbl->TryGetFrameReferenceBySourceId(This, sourceId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIMultiSourceMediaFrameReference_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Capture.Frames.IVideoMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.VideoMediaFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IVideoMediaFrame[] = L"Windows.Media.Capture.Frames.IVideoMediaFrame";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FrameReference)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameReference** value);
    HRESULT (STDMETHODCALLTYPE* get_VideoFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_SoftwareBitmap)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CGraphics_CImaging_CISoftwareBitmap** value);
    HRESULT (STDMETHODCALLTYPE* get_Direct3DSurface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CGraphics_CDirectX_CDirect3D11_CIDirect3DSurface** value);
    HRESULT (STDMETHODCALLTYPE* get_CameraIntrinsics)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CMedia_CDevices_CCore_CICameraIntrinsics** value);
    HRESULT (STDMETHODCALLTYPE* get_InfraredMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIInfraredMediaFrame** value);
    HRESULT (STDMETHODCALLTYPE* get_DepthMediaFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrame** value);
    HRESULT (STDMETHODCALLTYPE* GetVideoFrame)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame* This,
        __x_ABI_CWindows_CMedia_CIVideoFrame** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_FrameReference(This, value) \
    ((This)->lpVtbl->get_FrameReference(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_VideoFormat(This, value) \
    ((This)->lpVtbl->get_VideoFormat(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_SoftwareBitmap(This, value) \
    ((This)->lpVtbl->get_SoftwareBitmap(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_Direct3DSurface(This, value) \
    ((This)->lpVtbl->get_Direct3DSurface(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_CameraIntrinsics(This, value) \
    ((This)->lpVtbl->get_CameraIntrinsics(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_InfraredMediaFrame(This, value) \
    ((This)->lpVtbl->get_InfraredMediaFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_get_DepthMediaFrame(This, value) \
    ((This)->lpVtbl->get_DepthMediaFrame(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_GetVideoFrame(This, value) \
    ((This)->lpVtbl->GetVideoFrame(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Capture.Frames.IVideoMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Capture.Frames.VideoMediaFrameFormat
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Capture_Frames_IVideoMediaFrameFormat[] = L"Windows.Media.Capture.Frames.IVideoMediaFrameFormat";
typedef struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormatVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MediaFrameFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_DepthFormat)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        __x_ABI_CWindows_CMedia_CCapture_CFrames_CIDepthMediaFrameFormat** value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormatVtbl;

interface __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormatVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_get_MediaFrameFormat(This, value) \
    ((This)->lpVtbl->get_MediaFrameFormat(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_get_DepthFormat(This, value) \
    ((This)->lpVtbl->get_DepthFormat(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat;
#endif /* !defined(____x_ABI_CWindows_CMedia_CCapture_CFrames_CIVideoMediaFrameFormat_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.AudioMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IAudioMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_AudioMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_AudioMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_AudioMediaFrame[] = L"Windows.Media.Capture.Frames.AudioMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.Capture.Frames.BufferMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IBufferMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_BufferMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_BufferMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_BufferMediaFrame[] = L"Windows.Media.Capture.Frames.BufferMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.DepthMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IDepthMediaFrame ** Default Interface **
 *    Windows.Media.Capture.Frames.IDepthMediaFrame2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_DepthMediaFrame[] = L"Windows.Media.Capture.Frames.DepthMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.DepthMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IDepthMediaFrameFormat ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrameFormat_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_DepthMediaFrameFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_DepthMediaFrameFormat[] = L"Windows.Media.Capture.Frames.DepthMediaFrameFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.InfraredMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IInfraredMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_InfraredMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_InfraredMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_InfraredMediaFrame[] = L"Windows.Media.Capture.Frames.InfraredMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameArrivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameArrivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameArrivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.MediaFrameArrivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameFormat ** Default Interface **
 *    Windows.Media.Capture.Frames.IMediaFrameFormat2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameFormat_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameFormat[] = L"Windows.Media.Capture.Frames.MediaFrameFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Capture.Frames.IMediaFrameReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReader_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameReader[] = L"Windows.Media.Capture.Frames.MediaFrameReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameReference ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Capture.Frames.IMediaFrameReference2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReference_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameReference[] = L"Windows.Media.Capture.Frames.MediaFrameReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSource ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSource[] = L"Windows.Media.Capture.Frames.MediaFrameSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceController
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceController ** Default Interface **
 *    Windows.Media.Capture.Frames.IMediaFrameSourceController2
 *    Windows.Media.Capture.Frames.IMediaFrameSourceController3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceController_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceController_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceController[] = L"Windows.Media.Capture.Frames.MediaFrameSourceController";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceGetPropertyResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGetPropertyResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGetPropertyResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceGetPropertyResult[] = L"Windows.Media.Capture.Frames.MediaFrameSourceGetPropertyResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Capture.Frames.IMediaFrameSourceGroupStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGroup_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceGroup[] = L"Windows.Media.Capture.Frames.MediaFrameSourceGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MediaFrameSourceInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo ** Default Interface **
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo2
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo3
 *    Windows.Media.Capture.Frames.IMediaFrameSourceInfo4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceInfo_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MediaFrameSourceInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MediaFrameSourceInfo[] = L"Windows.Media.Capture.Frames.MediaFrameSourceInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameArrivedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameArrivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameArrivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MultiSourceMediaFrameArrivedEventArgs[] = L"Windows.Media.Capture.Frames.MultiSourceMediaFrameArrivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Capture.Frames.MultiSourceMediaFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameReader2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReader_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MultiSourceMediaFrameReader[] = L"Windows.Media.Capture.Frames.MultiSourceMediaFrameReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Capture.Frames.MultiSourceMediaFrameReference
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IMultiSourceMediaFrameReference ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReference_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_MultiSourceMediaFrameReference_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_MultiSourceMediaFrameReference[] = L"Windows.Media.Capture.Frames.MultiSourceMediaFrameReference";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Capture.Frames.VideoMediaFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IVideoMediaFrame ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrame_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_VideoMediaFrame[] = L"Windows.Media.Capture.Frames.VideoMediaFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Media.Capture.Frames.VideoMediaFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Capture.Frames.IVideoMediaFrameFormat ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrameFormat_DEFINED
#define RUNTIMECLASS_Windows_Media_Capture_Frames_VideoMediaFrameFormat_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Capture_Frames_VideoMediaFrameFormat[] = L"Windows.Media.Capture.Frames.VideoMediaFrameFormat";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Ecapture2Eframes_p_h__

#endif // __windows2Emedia2Ecapture2Eframes_h__
