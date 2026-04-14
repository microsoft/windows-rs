
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
#ifndef __windows2Emedia2Estreaming2Eadaptive_h__
#define __windows2Emedia2Estreaming2Eadaptive_h__
#ifndef __windows2Emedia2Estreaming2Eadaptive_p_h__
#define __windows2Emedia2Estreaming2Eadaptive_p_h__


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
#include "Windows.Media.Core.h"
#include "Windows.Storage.Streams.h"
#include "Windows.Web.Http.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSource;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSource2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSource3;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource3

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceAdvancedSettings;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceAdvancedSettings

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceCorrelatedTimes;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceCorrelatedTimes

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceCreationResult;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceCreationResult

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceCreationResult2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceCreationResult2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDiagnosticAvailableEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnosticAvailableEventArgs

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDiagnosticAvailableEventArgs2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnosticAvailableEventArgs2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDiagnosticAvailableEventArgs3;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnosticAvailableEventArgs3

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDiagnostics;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnostics

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadBitrateChangedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadBitrateChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadCompletedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadCompletedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadCompletedEventArgs2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadCompletedEventArgs2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadCompletedEventArgs3;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadCompletedEventArgs3

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadFailedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadFailedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadFailedEventArgs2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadFailedEventArgs2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadFailedEventArgs3;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadFailedEventArgs3

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadRequestedDeferral;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadRequestedDeferral

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadRequestedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadRequestedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadRequestedEventArgs2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadRequestedEventArgs2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadRequestedEventArgs3;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadRequestedEventArgs3

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadResult;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadResult

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadResult2;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2 ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadResult2

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceDownloadStatistics;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadStatistics

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    interface IAdaptiveMediaSourceStatics;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceStatics

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceCreationResult;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("be0bcc1d-d606-59d2-b2f9-ff204543da12"))
IAsyncOperation<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationResult*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationResult*> __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd68cc00-724c-5a76-a437-1464ebdda4ac"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationResult*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceCreationResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_UINT32_USE
#define DEF___FIIterator_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f06a2739-9443-5ef0-b284-dc5aff3e7d10"))
IIterator<UINT32> : IIterator_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<UINT32> __FIIterator_1_UINT32_t;
#define __FIIterator_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterator_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_UINT32_USE */



#ifndef DEF___FIIterable_1_UINT32_USE
#define DEF___FIIterable_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("421d4b91-b13b-5f37-ae54-b5249bd80539"))
IIterable<UINT32> : IIterable_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<UINT32> __FIIterable_1_UINT32_t;
#define __FIIterable_1_UINT32 ABI::Windows::Foundation::Collections::__FIIterable_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_UINT32_USE */



#ifndef DEF___FIVectorView_1_UINT32_USE
#define DEF___FIVectorView_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e5ce1a07-8d33-5007-ba64-7d2508ccf85c"))
IVectorView<UINT32> : IVectorView_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<UINT32> __FIVectorView_1_UINT32_t;
#define __FIVectorView_1_UINT32 ABI::Windows::Foundation::Collections::__FIVectorView_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_UINT32_USE */



#ifndef DEF___FIReference_1_double_USE
#define DEF___FIReference_1_double_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2f2d6c29-5473-5f3e-92e7-96572bb990e2"))
IReference<double> : IReference_impl<double>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Double>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<double> __FIReference_1_double_t;
#define __FIReference_1_double ABI::Windows::Foundation::__FIReference_1_double_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_double_USE */



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



#ifndef DEF___FIReference_1_UINT64_USE
#define DEF___FIReference_1_UINT64_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6755e376-53bb-568b-a11d-17239868309e"))
IReference<UINT64> : IReference_impl<UINT64>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<UInt64>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<UINT64> __FIReference_1_UINT64_t;
#define __FIReference_1_UINT64 ABI::Windows::Foundation::__FIReference_1_UINT64_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_UINT64_USE */


namespace ABI {
    namespace Windows {
        namespace Foundation {
            typedef struct DateTime DateTime;
        } /* Foundation */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#define DEF___FIReference_1_Windows__CFoundation__CDateTime_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5541d8a7-497c-5aa4-86fc-7713adbf2a2c"))
IReference<struct ABI::Windows::Foundation::DateTime> : IReference_impl<struct ABI::Windows::Foundation::DateTime>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Foundation.DateTime>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<struct ABI::Windows::Foundation::DateTime> __FIReference_1_Windows__CFoundation__CDateTime_t;
#define __FIReference_1_Windows__CFoundation__CDateTime ABI::Windows::Foundation::__FIReference_1_Windows__CFoundation__CDateTime_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CFoundation__CDateTime_USE */

#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

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
            namespace Streaming {
                namespace Adaptive {
                    typedef enum AdaptiveMediaSourceResourceType : int AdaptiveMediaSourceResourceType;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_USE
#define DEF___FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("74c8c3aa-de03-5bf0-aae8-aa8b692066b3"))
IReference<enum ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceResourceType> : IReference_impl<enum ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceResourceType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceResourceType> __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_t;
#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType ABI::Windows::Foundation::__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSource;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadBitrateChangedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ad268caf-7da0-5ad4-8585-ceeb903dbd50"))
ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadBitrateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadBitrateChangedEventArgs*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadBitrateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Streaming.Adaptive.AdaptiveMediaSource, Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadBitrateChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadCompletedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cef775dd-25b2-5588-8d51-dacdea660a7d"))
ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadCompletedEventArgs*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Streaming.Adaptive.AdaptiveMediaSource, Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadCompletedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadFailedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6ee5cc44-49ad-5138-ab47-f5c075a2bc34"))
ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadFailedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadFailedEventArgs*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadFailedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Streaming.Adaptive.AdaptiveMediaSource, Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadFailedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadRequestedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3b7dbf1-fd8e-589e-9c7f-ba67397e50cd"))
ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadRequestedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadRequestedEventArgs*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadRequestedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Streaming.Adaptive.AdaptiveMediaSource, Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadRequestedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourcePlaybackBitrateChangedEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df4f4e89-6173-539b-94d8-69b7cb7578a7"))
ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourcePlaybackBitrateChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourcePlaybackBitrateChangedEventArgs*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Streaming.Adaptive.AdaptiveMediaSource, Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSource*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourcePlaybackBitrateChangedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDiagnostics;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDiagnosticAvailableEventArgs;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("fd4ce6b5-7c3b-58f4-9efc-1d9ee6a09d21"))
ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnostics*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnosticAvailableEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnostics*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnostics*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnosticAvailableEventArgs*, ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnosticAvailableEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics, Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnostics*, ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnosticAvailableEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

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

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaSource ABI::Windows::Media::Core::IMediaSource

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__

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

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpClient;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpClient;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpClient ABI::Windows::Web::Http::IHttpClient

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                class HttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Web {
            namespace Http {
                interface IHttpResponseMessage;
            } /* Http */
        } /* Web */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage ABI::Windows::Web::Http::IHttpResponseMessage

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    typedef enum AdaptiveMediaSourceCreationStatus : int AdaptiveMediaSourceCreationStatus;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    typedef enum AdaptiveMediaSourceDiagnosticType : int AdaptiveMediaSourceDiagnosticType;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    typedef enum AdaptiveMediaSourceDownloadBitrateChangedReason : int AdaptiveMediaSourceDownloadBitrateChangedReason;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceAdvancedSettings;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceCorrelatedTimes;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadRequestedDeferral;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadResult;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    class AdaptiveMediaSourceDownloadStatistics;
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    enum AdaptiveMediaSourceCreationStatus : int
                    {
                        AdaptiveMediaSourceCreationStatus_Success = 0,
                        AdaptiveMediaSourceCreationStatus_ManifestDownloadFailure = 1,
                        AdaptiveMediaSourceCreationStatus_ManifestParseFailure = 2,
                        AdaptiveMediaSourceCreationStatus_UnsupportedManifestContentType = 3,
                        AdaptiveMediaSourceCreationStatus_UnsupportedManifestVersion = 4,
                        AdaptiveMediaSourceCreationStatus_UnsupportedManifestProfile = 5,
                        AdaptiveMediaSourceCreationStatus_UnknownFailure = 6,
                    };
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    enum AdaptiveMediaSourceDiagnosticType : int
                    {
                        AdaptiveMediaSourceDiagnosticType_ManifestUnchangedUponReload = 0,
                        AdaptiveMediaSourceDiagnosticType_ManifestMismatchUponReload = 1,
                        AdaptiveMediaSourceDiagnosticType_ManifestSignaledEndOfLiveEventUponReload = 2,
                        AdaptiveMediaSourceDiagnosticType_MediaSegmentSkipped = 3,
                        AdaptiveMediaSourceDiagnosticType_ResourceNotFound = 4,
                        AdaptiveMediaSourceDiagnosticType_ResourceTimedOut = 5,
                        AdaptiveMediaSourceDiagnosticType_ResourceParsingError = 6,
                        AdaptiveMediaSourceDiagnosticType_BitrateDisabled = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        AdaptiveMediaSourceDiagnosticType_FatalMediaSourceError = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    };
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    enum AdaptiveMediaSourceDownloadBitrateChangedReason : int
                    {
                        AdaptiveMediaSourceDownloadBitrateChangedReason_SufficientInboundBitsPerSecond = 0,
                        AdaptiveMediaSourceDownloadBitrateChangedReason_InsufficientInboundBitsPerSecond = 1,
                        AdaptiveMediaSourceDownloadBitrateChangedReason_LowBufferLevel = 2,
                        AdaptiveMediaSourceDownloadBitrateChangedReason_PositionChanged = 3,
                        AdaptiveMediaSourceDownloadBitrateChangedReason_TrackSelectionChanged = 4,
                        AdaptiveMediaSourceDownloadBitrateChangedReason_DesiredBitratesChanged = 5,
                        AdaptiveMediaSourceDownloadBitrateChangedReason_ErrorInPreviousBitrate = 6,
                    };
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    enum AdaptiveMediaSourceResourceType : int
                    {
                        AdaptiveMediaSourceResourceType_Manifest = 0,
                        AdaptiveMediaSourceResourceType_InitializationSegment = 1,
                        AdaptiveMediaSourceResourceType_MediaSegment = 2,
                        AdaptiveMediaSourceResourceType_Key = 3,
                        AdaptiveMediaSourceResourceType_InitializationVector = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                        AdaptiveMediaSourceResourceType_MediaSegmentIndex = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    };
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Core.IMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSource[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("4c7332ef-d39f-4396-b4d9-043957a7c964")
                    IAdaptiveMediaSource : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsLive(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredLiveOffset(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DesiredLiveOffset(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InitialBitrate(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InitialBitrate(
                            UINT32 value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentDownloadBitrate(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CurrentPlaybackBitrate(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AvailableBitrates(
                            __FIVectorView_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredMinBitrate(
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DesiredMinBitrate(
                            __FIReference_1_UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredMaxBitrate(
                            __FIReference_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DesiredMaxBitrate(
                            __FIReference_1_UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioOnlyPlayback(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InboundBitsPerSecond(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InboundBitsPerSecondWindow(
                            ABI::Windows::Foundation::TimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InboundBitsPerSecondWindow(
                            ABI::Windows::Foundation::TimeSpan value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DownloadBitrateChanged(
                            __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DownloadBitrateChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_PlaybackBitrateChanged(
                            __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_PlaybackBitrateChanged(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DownloadRequested(
                            __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DownloadRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DownloadCompleted(
                            __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DownloadCompleted(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DownloadFailed(
                            __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DownloadFailed(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSource = __uuidof(IAdaptiveMediaSource);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSource2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("17890342-6760-4bb9-a58a-f7aa98b08c0e")
                    IAdaptiveMediaSource2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AdvancedSettings(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceAdvancedSettings** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSource2 = __uuidof(IAdaptiveMediaSource2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSource3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("ba7023fd-c334-461b-a36e-c99f54f7174a")
                    IAdaptiveMediaSource3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MinLiveOffset(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MaxSeekableWindowSize(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredSeekableWindowSize(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DesiredSeekableWindowSize(
                            __FIReference_1_Windows__CFoundation__CTimeSpan* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Diagnostics(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDiagnostics** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetCorrelatedTimes(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceCorrelatedTimes** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSource3 = __uuidof(IAdaptiveMediaSource3);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceAdvancedSettings[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("55db1680-1aeb-47dc-aa08-9a11610ba45a")
                    IAdaptiveMediaSourceAdvancedSettings : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AllSegmentsIndependent(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_AllSegmentsIndependent(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DesiredBitrateHeadroomRatio(
                            __FIReference_1_double** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DesiredBitrateHeadroomRatio(
                            __FIReference_1_double* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_BitrateDowngradeTriggerRatio(
                            __FIReference_1_double** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_BitrateDowngradeTriggerRatio(
                            __FIReference_1_double* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceAdvancedSettings = __uuidof(IAdaptiveMediaSourceAdvancedSettings);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceCorrelatedTimes[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("05108787-e032-48e1-ab8d-002b0b3051df")
                    IAdaptiveMediaSourceCorrelatedTimes : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PresentationTimeStamp(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProgramDateTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceCorrelatedTimes = __uuidof(IAdaptiveMediaSourceCorrelatedTimes);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceCreationResult[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("4686b6b2-800f-4e31-9093-76d4782013e7")
                    IAdaptiveMediaSourceCreationResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Status(
                            ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceCreationStatus* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MediaSource(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSource** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HttpResponseMessage(
                            ABI::Windows::Web::Http::IHttpResponseMessage** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceCreationResult = __uuidof(IAdaptiveMediaSourceCreationResult);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceCreationResult2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("1c3243bf-1c44-404b-a201-df45ac7898e8")
                    IAdaptiveMediaSourceCreationResult2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceCreationResult2 = __uuidof(IAdaptiveMediaSourceCreationResult2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnosticAvailableEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("3af64f06-6d9c-494a-b7a9-b3a5dee6ad68")
                    IAdaptiveMediaSourceDiagnosticAvailableEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DiagnosticType(
                            ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDiagnosticType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RequestId(
                            __FIReference_1_int** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SegmentId(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceType(
                            __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeOffset(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeLength(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Bitrate(
                            __FIReference_1_UINT32** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDiagnosticAvailableEventArgs = __uuidof(IAdaptiveMediaSourceDiagnosticAvailableEventArgs);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnosticAvailableEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("8c6dd857-16a5-4d9f-810e-00bd901b3ef9")
                    IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDiagnosticAvailableEventArgs2 = __uuidof(IAdaptiveMediaSourceDiagnosticAvailableEventArgs2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnosticAvailableEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("c3650cd5-daeb-4103-84da-68769ad513ff")
                    IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceDuration(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceContentType(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDiagnosticAvailableEventArgs3 = __uuidof(IAdaptiveMediaSourceDiagnosticAvailableEventArgs3);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnostics[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("9b24ee68-962e-448c-aebf-b29b56098e23")
                    IAdaptiveMediaSourceDiagnostics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE add_DiagnosticAvailable(
                            __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DiagnosticAvailable(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDiagnostics = __uuidof(IAdaptiveMediaSourceDiagnostics);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("670c0a44-e04e-4eff-816a-17399f78f4ba")
                    IAdaptiveMediaSourceDownloadBitrateChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_OldValue(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewValue(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadBitrateChangedEventArgs = __uuidof(IAdaptiveMediaSourceDownloadBitrateChangedEventArgs);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("f3f1f444-96ae-4de0-b540-2b3246e6968c")
                    IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Reason(
                            ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceDownloadBitrateChangedReason* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 = __uuidof(IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadCompletedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("19240dc3-5b37-4a1a-8970-d621cb6ca83b")
                    IAdaptiveMediaSourceDownloadCompletedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceType(
                            ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceResourceType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeOffset(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeLength(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HttpResponseMessage(
                            ABI::Windows::Web::Http::IHttpResponseMessage** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadCompletedEventArgs = __uuidof(IAdaptiveMediaSourceDownloadCompletedEventArgs);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadCompletedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("704744c4-964a-40e4-af95-9177dd6dfa00")
                    IAdaptiveMediaSourceDownloadCompletedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RequestId(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Statistics(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadStatistics** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadCompletedEventArgs2 = __uuidof(IAdaptiveMediaSourceDownloadCompletedEventArgs2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadCompletedEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("0f8a8bd1-93b2-47c6-badc-8be2c8f7f6e8")
                    IAdaptiveMediaSourceDownloadCompletedEventArgs3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceDuration(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceContentType(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadCompletedEventArgs3 = __uuidof(IAdaptiveMediaSourceDownloadCompletedEventArgs3);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadFailedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("37739048-f4ab-40a4-b135-c6dfd8bd7ff1")
                    IAdaptiveMediaSourceDownloadFailedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceType(
                            ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceResourceType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeOffset(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeLength(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HttpResponseMessage(
                            ABI::Windows::Web::Http::IHttpResponseMessage** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadFailedEventArgs = __uuidof(IAdaptiveMediaSourceDownloadFailedEventArgs);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadFailedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("70919568-967c-4986-90c5-c6fc4b31e2d8")
                    IAdaptiveMediaSourceDownloadFailedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RequestId(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                            HRESULT* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Statistics(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadStatistics** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadFailedEventArgs2 = __uuidof(IAdaptiveMediaSourceDownloadFailedEventArgs2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadFailedEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("d0354549-1132-4a10-915a-c2211b5b9409")
                    IAdaptiveMediaSourceDownloadFailedEventArgs3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceDuration(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceContentType(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadFailedEventArgs3 = __uuidof(IAdaptiveMediaSourceDownloadFailedEventArgs3);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedDeferral[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("05c68f64-fa20-4dbd-9821-4bf4c9bf77ab")
                    IAdaptiveMediaSourceDownloadRequestedDeferral : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadRequestedDeferral = __uuidof(IAdaptiveMediaSourceDownloadRequestedDeferral);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("c83fdffd-44a9-47a2-bf96-03398b4bfaaf")
                    IAdaptiveMediaSourceDownloadRequestedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceType(
                            ABI::Windows::Media::Streaming::Adaptive::AdaptiveMediaSourceResourceType* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeOffset(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeLength(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Result(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadResult** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Media::Streaming::Adaptive::IAdaptiveMediaSourceDownloadRequestedDeferral** deferral
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadRequestedEventArgs = __uuidof(IAdaptiveMediaSourceDownloadRequestedEventArgs);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("b37d8bfe-aa44-4d82-825b-611de3bcfecb")
                    IAdaptiveMediaSourceDownloadRequestedEventArgs2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_RequestId(
                            INT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Position(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadRequestedEventArgs2 = __uuidof(IAdaptiveMediaSourceDownloadRequestedEventArgs2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("333c50fd-4f62-4481-ab44-1e47b0574225")
                    IAdaptiveMediaSourceDownloadRequestedEventArgs3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceDuration(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceContentType(
                            HSTRING* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadRequestedEventArgs3 = __uuidof(IAdaptiveMediaSourceDownloadRequestedEventArgs3);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadResult[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("f4afdc73-bcee-4a6a-9f0a-fec41e2339b0")
                    IAdaptiveMediaSourceDownloadResult : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ResourceUri(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InputStream(
                            ABI::Windows::Storage::Streams::IInputStream** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_InputStream(
                            ABI::Windows::Storage::Streams::IInputStream* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Buffer(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Buffer(
                            ABI::Windows::Storage::Streams::IBuffer* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ContentType(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContentType(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExtendedStatus(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ExtendedStatus(
                            UINT32 value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadResult = __uuidof(IAdaptiveMediaSourceDownloadResult);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadResult2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("15552cb7-7b80-4ac4-8660-a4b97f7c70f0")
                    IAdaptiveMediaSourceDownloadResult2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeOffset(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ResourceByteRangeOffset(
                            __FIReference_1_UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResourceByteRangeLength(
                            __FIReference_1_UINT64** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ResourceByteRangeLength(
                            __FIReference_1_UINT64* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadResult2 = __uuidof(IAdaptiveMediaSourceDownloadResult2);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadStatistics[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("a306cefb-e96a-4dff-a9b8-1ae08c01ae98")
                    IAdaptiveMediaSourceDownloadStatistics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContentBytesReceivedCount(
                            UINT64* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TimeToHeadersReceived(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TimeToFirstByteReceived(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_TimeToLastByteReceived(
                            __FIReference_1_Windows__CFoundation__CTimeSpan** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceDownloadStatistics = __uuidof(IAdaptiveMediaSourceDownloadStatistics);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("23a29f6d-7dda-4a51-87a9-6fa8c5b292be")
                    IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_OldValue(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_NewValue(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_AudioOnly(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs = __uuidof(IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceStatics[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Streaming {
                namespace Adaptive {
                    MIDL_INTERFACE("50a6bd5d-66ef-4cd3-9579-9e660507dc3f")
                    IAdaptiveMediaSourceStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE IsContentTypeSupported(
                            HSTRING contentType,
                            boolean* result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFromUriAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFromUriWithDownloaderAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            ABI::Windows::Web::Http::IHttpClient* httpClient,
                            __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFromStreamAsync(
                            ABI::Windows::Storage::Streams::IInputStream* stream,
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            HSTRING contentType,
                            __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFromStreamWithDownloaderAsync(
                            ABI::Windows::Storage::Streams::IInputStream* stream,
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            HSTRING contentType,
                            ABI::Windows::Web::Http::IHttpClient* httpClient,
                            __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IAdaptiveMediaSourceStatics = __uuidof(IAdaptiveMediaSourceStatics);
                } /* Adaptive */
            } /* Streaming */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource ** Default Interface **
 *    Windows.Media.Core.IMediaSource
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSource[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceAdvancedSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceAdvancedSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceAdvancedSettings[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCorrelatedTimes_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCorrelatedTimes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCorrelatedTimes[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCreationResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCreationResult[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnosticAvailableEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnosticAvailableEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnosticAvailableEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnostics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnostics[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadBitrateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadBitrateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadCompletedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadFailedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedDeferral[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadResult[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadStatistics_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadStatistics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadStatistics[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourcePlaybackBitrateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourcePlaybackBitrateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourcePlaybackBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2 __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics;

#endif // ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterator_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterator_1_UINT32 __FIIterator_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_UINT32;

typedef struct __FIIterator_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_UINT32* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_UINT32Vtbl;

interface __FIIterator_1_UINT32
{
    CONST_VTBL struct __FIIterator_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_UINT32_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_UINT32_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_UINT32_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_UINT32_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_UINT32_INTERFACE_DEFINED__)
#define ____FIIterable_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIIterable_1_UINT32 __FIIterable_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_UINT32;

typedef struct __FIIterable_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_UINT32* This,
        __FIIterator_1_UINT32** result);

    END_INTERFACE
} __FIIterable_1_UINT32Vtbl;

interface __FIIterable_1_UINT32
{
    CONST_VTBL struct __FIIterable_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_UINT32_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIVectorView_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_UINT32 __FIVectorView_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_UINT32;

typedef struct __FIVectorView_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_UINT32Vtbl;

interface __FIVectorView_1_UINT32
{
    CONST_VTBL struct __FIVectorView_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_UINT32_INTERFACE_DEFINED__

#if !defined(____FIReference_1_double_INTERFACE_DEFINED__)
#define ____FIReference_1_double_INTERFACE_DEFINED__

typedef interface __FIReference_1_double __FIReference_1_double;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_double;

typedef struct __FIReference_1_doubleVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_double* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_double* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_double* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_double* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_double* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_double* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_double* This,
        DOUBLE* result);

    END_INTERFACE
} __FIReference_1_doubleVtbl;

interface __FIReference_1_double
{
    CONST_VTBL struct __FIReference_1_doubleVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_double_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_double_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_double_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_double_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_double_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_double_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_double_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_double_INTERFACE_DEFINED__

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

#if !defined(____FIReference_1_UINT64_INTERFACE_DEFINED__)
#define ____FIReference_1_UINT64_INTERFACE_DEFINED__

typedef interface __FIReference_1_UINT64 __FIReference_1_UINT64;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_UINT64;

typedef struct __FIReference_1_UINT64Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_UINT64* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_UINT64* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_UINT64* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_UINT64* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_UINT64* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_UINT64* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_UINT64* This,
        UINT64* result);

    END_INTERFACE
} __FIReference_1_UINT64Vtbl;

interface __FIReference_1_UINT64
{
    CONST_VTBL struct __FIReference_1_UINT64Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_UINT64_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_UINT64_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_UINT64_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_UINT64_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_UINT64_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_UINT64_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_UINT64_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_UINT64_INTERFACE_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CDateTime __x_ABI_CWindows_CFoundation_CDateTime;

#if WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CFoundation__CDateTime __FIReference_1_Windows__CFoundation__CDateTime;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CFoundation__CDateTime;

typedef struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CFoundation__CDateTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CFoundation__CDateTime* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* result);

    END_INTERFACE
} __FIReference_1_Windows__CFoundation__CDateTimeVtbl;

interface __FIReference_1_Windows__CFoundation__CDateTime
{
    CONST_VTBL struct __FIReference_1_Windows__CFoundation__CDateTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CFoundation__CDateTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CFoundation__CDateTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CFoundation__CDateTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CFoundation__CDateTime_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CFoundation__CDateTime_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION >= 0x10000

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

typedef enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType;

typedef struct __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType* result);

    END_INTERFACE
} __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceTypeVtbl;

interface __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType
{
    CONST_VTBL struct __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* sender,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* sender,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* sender,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* sender,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* sender,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* sender,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIClosable __x_ABI_CWindows_CFoundation_CIClosable;

#endif // ____x_ABI_CWindows_CFoundation_CIClosable_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaSource __x_ABI_CWindows_CMedia_CCore_CIMediaSource;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpClient __x_ABI_CWindows_CWeb_CHttp_CIHttpClient;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage;

#endif // ____x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceCreationStatus __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceCreationStatus;

typedef enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDiagnosticType __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDiagnosticType;

typedef enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDownloadBitrateChangedReason __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDownloadBitrateChangedReason;

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceCreationStatus
{
    AdaptiveMediaSourceCreationStatus_Success = 0,
    AdaptiveMediaSourceCreationStatus_ManifestDownloadFailure = 1,
    AdaptiveMediaSourceCreationStatus_ManifestParseFailure = 2,
    AdaptiveMediaSourceCreationStatus_UnsupportedManifestContentType = 3,
    AdaptiveMediaSourceCreationStatus_UnsupportedManifestVersion = 4,
    AdaptiveMediaSourceCreationStatus_UnsupportedManifestProfile = 5,
    AdaptiveMediaSourceCreationStatus_UnknownFailure = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDiagnosticType
{
    AdaptiveMediaSourceDiagnosticType_ManifestUnchangedUponReload = 0,
    AdaptiveMediaSourceDiagnosticType_ManifestMismatchUponReload = 1,
    AdaptiveMediaSourceDiagnosticType_ManifestSignaledEndOfLiveEventUponReload = 2,
    AdaptiveMediaSourceDiagnosticType_MediaSegmentSkipped = 3,
    AdaptiveMediaSourceDiagnosticType_ResourceNotFound = 4,
    AdaptiveMediaSourceDiagnosticType_ResourceTimedOut = 5,
    AdaptiveMediaSourceDiagnosticType_ResourceParsingError = 6,
    AdaptiveMediaSourceDiagnosticType_BitrateDisabled = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    AdaptiveMediaSourceDiagnosticType_FatalMediaSourceError = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedReason
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDownloadBitrateChangedReason
{
    AdaptiveMediaSourceDownloadBitrateChangedReason_SufficientInboundBitsPerSecond = 0,
    AdaptiveMediaSourceDownloadBitrateChangedReason_InsufficientInboundBitsPerSecond = 1,
    AdaptiveMediaSourceDownloadBitrateChangedReason_LowBufferLevel = 2,
    AdaptiveMediaSourceDownloadBitrateChangedReason_PositionChanged = 3,
    AdaptiveMediaSourceDownloadBitrateChangedReason_TrackSelectionChanged = 4,
    AdaptiveMediaSourceDownloadBitrateChangedReason_DesiredBitratesChanged = 5,
    AdaptiveMediaSourceDownloadBitrateChangedReason_ErrorInPreviousBitrate = 6,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceResourceType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType
{
    AdaptiveMediaSourceResourceType_Manifest = 0,
    AdaptiveMediaSourceResourceType_InitializationSegment = 1,
    AdaptiveMediaSourceResourceType_MediaSegment = 2,
    AdaptiveMediaSourceResourceType_Key = 3,
    AdaptiveMediaSourceResourceType_InitializationVector = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    AdaptiveMediaSourceResourceType_MediaSegmentIndex = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Core.IMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSource[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsLive)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredLiveOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredLiveOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* get_InitialBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_InitialBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentDownloadBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentPlaybackBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AvailableBitrates)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FIVectorView_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredMinBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredMinBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FIReference_1_UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredMaxBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FIReference_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredMaxBitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FIReference_1_UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AudioOnlyPlayback)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_InboundBitsPerSecond)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_InboundBitsPerSecondWindow)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* put_InboundBitsPerSecondWindow)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan value);
    HRESULT (STDMETHODCALLTYPE* add_DownloadBitrateChanged)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadBitrateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadBitrateChanged)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PlaybackBitrateChanged)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PlaybackBitrateChanged)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DownloadRequested)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadRequestedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadRequested)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DownloadCompleted)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadCompleted)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DownloadFailed)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSource_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDownloadFailedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DownloadFailed)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_IsLive(This, value) \
    ((This)->lpVtbl->get_IsLive(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_DesiredLiveOffset(This, value) \
    ((This)->lpVtbl->get_DesiredLiveOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_put_DesiredLiveOffset(This, value) \
    ((This)->lpVtbl->put_DesiredLiveOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_InitialBitrate(This, value) \
    ((This)->lpVtbl->get_InitialBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_put_InitialBitrate(This, value) \
    ((This)->lpVtbl->put_InitialBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_CurrentDownloadBitrate(This, value) \
    ((This)->lpVtbl->get_CurrentDownloadBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_CurrentPlaybackBitrate(This, value) \
    ((This)->lpVtbl->get_CurrentPlaybackBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_AvailableBitrates(This, value) \
    ((This)->lpVtbl->get_AvailableBitrates(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_DesiredMinBitrate(This, value) \
    ((This)->lpVtbl->get_DesiredMinBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_put_DesiredMinBitrate(This, value) \
    ((This)->lpVtbl->put_DesiredMinBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_DesiredMaxBitrate(This, value) \
    ((This)->lpVtbl->get_DesiredMaxBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_put_DesiredMaxBitrate(This, value) \
    ((This)->lpVtbl->put_DesiredMaxBitrate(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_AudioOnlyPlayback(This, value) \
    ((This)->lpVtbl->get_AudioOnlyPlayback(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_InboundBitsPerSecond(This, value) \
    ((This)->lpVtbl->get_InboundBitsPerSecond(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_get_InboundBitsPerSecondWindow(This, value) \
    ((This)->lpVtbl->get_InboundBitsPerSecondWindow(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_put_InboundBitsPerSecondWindow(This, value) \
    ((This)->lpVtbl->put_InboundBitsPerSecondWindow(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_add_DownloadBitrateChanged(This, handler, token) \
    ((This)->lpVtbl->add_DownloadBitrateChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_remove_DownloadBitrateChanged(This, token) \
    ((This)->lpVtbl->remove_DownloadBitrateChanged(This, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_add_PlaybackBitrateChanged(This, handler, token) \
    ((This)->lpVtbl->add_PlaybackBitrateChanged(This, handler, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_remove_PlaybackBitrateChanged(This, token) \
    ((This)->lpVtbl->remove_PlaybackBitrateChanged(This, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_add_DownloadRequested(This, handler, token) \
    ((This)->lpVtbl->add_DownloadRequested(This, handler, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_remove_DownloadRequested(This, token) \
    ((This)->lpVtbl->remove_DownloadRequested(This, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_add_DownloadCompleted(This, handler, token) \
    ((This)->lpVtbl->add_DownloadCompleted(This, handler, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_remove_DownloadCompleted(This, token) \
    ((This)->lpVtbl->remove_DownloadCompleted(This, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_add_DownloadFailed(This, handler, token) \
    ((This)->lpVtbl->add_DownloadFailed(This, handler, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_remove_DownloadFailed(This, token) \
    ((This)->lpVtbl->remove_DownloadFailed(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSource2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AdvancedSettings)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_get_AdvancedSettings(This, value) \
    ((This)->lpVtbl->get_AdvancedSettings(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSource3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MinLiveOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_MaxSeekableWindowSize)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredSeekableWindowSize)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredSeekableWindowSize)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan* value);
    HRESULT (STDMETHODCALLTYPE* get_Diagnostics)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics** value);
    HRESULT (STDMETHODCALLTYPE* GetCorrelatedTimes)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_get_MinLiveOffset(This, value) \
    ((This)->lpVtbl->get_MinLiveOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_get_MaxSeekableWindowSize(This, value) \
    ((This)->lpVtbl->get_MaxSeekableWindowSize(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_get_DesiredSeekableWindowSize(This, value) \
    ((This)->lpVtbl->get_DesiredSeekableWindowSize(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_put_DesiredSeekableWindowSize(This, value) \
    ((This)->lpVtbl->put_DesiredSeekableWindowSize(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_get_Diagnostics(This, value) \
    ((This)->lpVtbl->get_Diagnostics(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_GetCorrelatedTimes(This, value) \
    ((This)->lpVtbl->GetCorrelatedTimes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceAdvancedSettings[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettingsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AllSegmentsIndependent)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllSegmentsIndependent)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_DesiredBitrateHeadroomRatio)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* put_DesiredBitrateHeadroomRatio)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        __FIReference_1_double* value);
    HRESULT (STDMETHODCALLTYPE* get_BitrateDowngradeTriggerRatio)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        __FIReference_1_double** value);
    HRESULT (STDMETHODCALLTYPE* put_BitrateDowngradeTriggerRatio)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings* This,
        __FIReference_1_double* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettingsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettingsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_get_AllSegmentsIndependent(This, value) \
    ((This)->lpVtbl->get_AllSegmentsIndependent(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_put_AllSegmentsIndependent(This, value) \
    ((This)->lpVtbl->put_AllSegmentsIndependent(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_get_DesiredBitrateHeadroomRatio(This, value) \
    ((This)->lpVtbl->get_DesiredBitrateHeadroomRatio(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_put_DesiredBitrateHeadroomRatio(This, value) \
    ((This)->lpVtbl->put_DesiredBitrateHeadroomRatio(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_get_BitrateDowngradeTriggerRatio(This, value) \
    ((This)->lpVtbl->get_BitrateDowngradeTriggerRatio(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_put_BitrateDowngradeTriggerRatio(This, value) \
    ((This)->lpVtbl->put_BitrateDowngradeTriggerRatio(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceAdvancedSettings_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceCorrelatedTimes[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_PresentationTimeStamp)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_ProgramDateTime)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimesVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_get_PresentationTimeStamp(This, value) \
    ((This)->lpVtbl->get_PresentationTimeStamp(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_get_ProgramDateTime(This, value) \
    ((This)->lpVtbl->get_ProgramDateTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCorrelatedTimes_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceCreationResult[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceCreationStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaSource)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSource** value);
    HRESULT (STDMETHODCALLTYPE* get_HttpResponseMessage)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResultVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_get_MediaSource(This, value) \
    ((This)->lpVtbl->get_MediaSource(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_get_HttpResponseMessage(This, value) \
    ((This)->lpVtbl->get_HttpResponseMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceCreationResult2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceCreationResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnosticAvailableEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DiagnosticType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDiagnosticType* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestId)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_SegmentId)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceResourceType** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceUri)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeLength)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_Bitrate)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs* This,
        __FIReference_1_UINT32** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_DiagnosticType(This, value) \
    ((This)->lpVtbl->get_DiagnosticType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_RequestId(This, value) \
    ((This)->lpVtbl->get_RequestId(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_SegmentId(This, value) \
    ((This)->lpVtbl->get_SegmentId(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_ResourceType(This, value) \
    ((This)->lpVtbl->get_ResourceType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_ResourceUri(This, value) \
    ((This)->lpVtbl->get_ResourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_ResourceByteRangeOffset(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_ResourceByteRangeLength(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeLength(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_get_Bitrate(This, value) \
    ((This)->lpVtbl->get_Bitrate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnosticAvailableEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnosticAvailableEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceDuration)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceContentType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_get_ResourceDuration(This, value) \
    ((This)->lpVtbl->get_ResourceDuration(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_get_ResourceContentType(This, value) \
    ((This)->lpVtbl->get_ResourceContentType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticAvailableEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDiagnostics[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_DiagnosticAvailable)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This,
        __FITypedEventHandler_2_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnostics_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceDiagnosticAvailableEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DiagnosticAvailable)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnosticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_add_DiagnosticAvailable(This, handler, token) \
    ((This)->lpVtbl->add_DiagnosticAvailable(This, handler, token))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_remove_DiagnosticAvailable(This, token) \
    ((This)->lpVtbl->remove_DiagnosticAvailable(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDiagnostics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OldValue)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NewValue)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_get_OldValue(This, value) \
    ((This)->lpVtbl->get_OldValue(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_get_NewValue(This, value) \
    ((This)->lpVtbl->get_NewValue(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceDownloadBitrateChangedReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadBitrateChangedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadCompletedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType* value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceUri)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeLength)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_HttpResponseMessage)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_get_ResourceType(This, value) \
    ((This)->lpVtbl->get_ResourceType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_get_ResourceUri(This, value) \
    ((This)->lpVtbl->get_ResourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_get_ResourceByteRangeOffset(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_get_ResourceByteRangeLength(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeLength(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_get_HttpResponseMessage(This, value) \
    ((This)->lpVtbl->get_HttpResponseMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadCompletedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestId)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Statistics)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics** value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_get_RequestId(This, value) \
    ((This)->lpVtbl->get_RequestId(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_get_Statistics(This, value) \
    ((This)->lpVtbl->get_Statistics(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadCompletedEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceDuration)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceContentType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_get_ResourceDuration(This, value) \
    ((This)->lpVtbl->get_ResourceDuration(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_get_ResourceContentType(This, value) \
    ((This)->lpVtbl->get_ResourceContentType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadCompletedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadFailedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType* value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceUri)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeLength)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_HttpResponseMessage)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs* This,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpResponseMessage** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_get_ResourceType(This, value) \
    ((This)->lpVtbl->get_ResourceType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_get_ResourceUri(This, value) \
    ((This)->lpVtbl->get_ResourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_get_ResourceByteRangeOffset(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_get_ResourceByteRangeLength(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeLength(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_get_HttpResponseMessage(This, value) \
    ((This)->lpVtbl->get_HttpResponseMessage(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadFailedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestId)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_Statistics)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics** value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_get_RequestId(This, value) \
    ((This)->lpVtbl->get_RequestId(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_get_Statistics(This, value) \
    ((This)->lpVtbl->get_Statistics(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadFailedEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceDuration)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceContentType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_get_ResourceDuration(This, value) \
    ((This)->lpVtbl->get_ResourceDuration(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_get_ResourceContentType(This, value) \
    ((This)->lpVtbl->get_ResourceContentType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadFailedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedDeferral[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferralVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CAdaptiveMediaSourceResourceType* value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceUri)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeLength)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs* This,
        __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedDeferral** deferral);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_get_ResourceType(This, value) \
    ((This)->lpVtbl->get_ResourceType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_get_ResourceUri(This, value) \
    ((This)->lpVtbl->get_ResourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_get_ResourceByteRangeOffset(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_get_ResourceByteRangeLength(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeLength(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedEventArgs2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequestId)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Position)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_get_RequestId(This, value) \
    ((This)->lpVtbl->get_RequestId(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_get_Position(This, value) \
    ((This)->lpVtbl->get_Position(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadRequestedEventArgs3[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceDuration)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceContentType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_get_ResourceDuration(This, value) \
    ((This)->lpVtbl->get_ResourceDuration(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_get_ResourceContentType(This, value) \
    ((This)->lpVtbl->get_ResourceContentType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadRequestedEventArgs3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadResult[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceUri)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_ResourceUri)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_InputStream)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);
    HRESULT (STDMETHODCALLTYPE* put_InputStream)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* value);
    HRESULT (STDMETHODCALLTYPE* get_Buffer)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);
    HRESULT (STDMETHODCALLTYPE* put_Buffer)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ContentType)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedStatus)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ExtendedStatus)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult* This,
        UINT32 value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResultVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_get_ResourceUri(This, value) \
    ((This)->lpVtbl->get_ResourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_put_ResourceUri(This, value) \
    ((This)->lpVtbl->put_ResourceUri(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_get_InputStream(This, value) \
    ((This)->lpVtbl->get_InputStream(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_put_InputStream(This, value) \
    ((This)->lpVtbl->put_InputStream(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_get_Buffer(This, value) \
    ((This)->lpVtbl->get_Buffer(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_put_Buffer(This, value) \
    ((This)->lpVtbl->put_Buffer(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_get_ContentType(This, value) \
    ((This)->lpVtbl->get_ContentType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_put_ContentType(This, value) \
    ((This)->lpVtbl->put_ContentType(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_get_ExtendedStatus(This, value) \
    ((This)->lpVtbl->get_ExtendedStatus(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_put_ExtendedStatus(This, value) \
    ((This)->lpVtbl->put_ExtendedStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadResult2[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* put_ResourceByteRangeOffset)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        __FIReference_1_UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceByteRangeLength)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        __FIReference_1_UINT64** value);
    HRESULT (STDMETHODCALLTYPE* put_ResourceByteRangeLength)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2* This,
        __FIReference_1_UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2Vtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_get_ResourceByteRangeOffset(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_put_ResourceByteRangeOffset(This, value) \
    ((This)->lpVtbl->put_ResourceByteRangeOffset(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_get_ResourceByteRangeLength(This, value) \
    ((This)->lpVtbl->get_ResourceByteRangeLength(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_put_ResourceByteRangeLength(This, value) \
    ((This)->lpVtbl->put_ResourceByteRangeLength(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceDownloadStatistics[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatisticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentBytesReceivedCount)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_TimeToHeadersReceived)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_TimeToFirstByteReceived)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);
    HRESULT (STDMETHODCALLTYPE* get_TimeToLastByteReceived)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics* This,
        __FIReference_1_Windows__CFoundation__CTimeSpan** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatisticsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatisticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_get_ContentBytesReceivedCount(This, value) \
    ((This)->lpVtbl->get_ContentBytesReceivedCount(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_get_TimeToHeadersReceived(This, value) \
    ((This)->lpVtbl->get_TimeToHeadersReceived(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_get_TimeToFirstByteReceived(This, value) \
    ((This)->lpVtbl->get_TimeToFirstByteReceived(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_get_TimeToLastByteReceived(This, value) \
    ((This)->lpVtbl->get_TimeToLastByteReceived(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceDownloadStatistics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OldValue)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_NewValue)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_AudioOnly)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_get_OldValue(This, value) \
    ((This)->lpVtbl->get_OldValue(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_get_NewValue(This, value) \
    ((This)->lpVtbl->get_NewValue(This, value))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_get_AudioOnly(This, value) \
    ((This)->lpVtbl->get_AudioOnly(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourcePlaybackBitrateChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Streaming_Adaptive_IAdaptiveMediaSourceStatics[] = L"Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics";
typedef struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* IsContentTypeSupported)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        HSTRING contentType,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* CreateFromUriAsync)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromUriWithDownloaderAsync)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpClient* httpClient,
        __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromStreamAsync)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* stream,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        HSTRING contentType,
        __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result);
    HRESULT (STDMETHODCALLTYPE* CreateFromStreamWithDownloaderAsync)(__x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* stream,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        HSTRING contentType,
        __x_ABI_CWindows_CWeb_CHttp_CIHttpClient* httpClient,
        __FIAsyncOperation_1_Windows__CMedia__CStreaming__CAdaptive__CAdaptiveMediaSourceCreationResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_IsContentTypeSupported(This, contentType, result) \
    ((This)->lpVtbl->IsContentTypeSupported(This, contentType, result))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_CreateFromUriAsync(This, uri, result) \
    ((This)->lpVtbl->CreateFromUriAsync(This, uri, result))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_CreateFromUriWithDownloaderAsync(This, uri, httpClient, result) \
    ((This)->lpVtbl->CreateFromUriWithDownloaderAsync(This, uri, httpClient, result))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_CreateFromStreamAsync(This, stream, uri, contentType, result) \
    ((This)->lpVtbl->CreateFromStreamAsync(This, stream, uri, contentType, result))

#define __x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_CreateFromStreamWithDownloaderAsync(This, stream, uri, contentType, httpClient, result) \
    ((This)->lpVtbl->CreateFromStreamWithDownloaderAsync(This, stream, uri, contentType, httpClient, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CStreaming_CAdaptive_CIAdaptiveMediaSourceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource ** Default Interface **
 *    Windows.Media.Core.IMediaSource
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSource3
 *    Windows.Foundation.IClosable
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSource_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSource_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSource[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSource";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceAdvancedSettings ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceAdvancedSettings_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceAdvancedSettings_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceAdvancedSettings[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceAdvancedSettings";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCorrelatedTimes ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCorrelatedTimes_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCorrelatedTimes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCorrelatedTimes[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCorrelatedTimes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceCreationResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCreationResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCreationResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceCreationResult[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceCreationResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnosticAvailableEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnosticAvailableEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnosticAvailableEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnosticAvailableEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnosticAvailableEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDiagnostics ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnostics_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnostics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDiagnostics[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDiagnostics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadBitrateChangedEventArgs2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadBitrateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadBitrateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadBitrateChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadCompletedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadCompletedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadCompletedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadFailedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadFailedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadFailedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadFailedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadFailedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedDeferral ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedDeferral_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedDeferral[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs2
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadRequestedEventArgs3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadRequestedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadRequestedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult ** Default Interface **
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadResult2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadResult_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadResult[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourceDownloadStatistics ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadStatistics_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadStatistics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourceDownloadStatistics[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourceDownloadStatistics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Streaming.Adaptive.IAdaptiveMediaSourcePlaybackBitrateChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourcePlaybackBitrateChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourcePlaybackBitrateChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Streaming_Adaptive_AdaptiveMediaSourcePlaybackBitrateChangedEventArgs[] = L"Windows.Media.Streaming.Adaptive.AdaptiveMediaSourcePlaybackBitrateChangedEventArgs";
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
#endif // __windows2Emedia2Estreaming2Eadaptive_p_h__

#endif // __windows2Emedia2Estreaming2Eadaptive_h__
