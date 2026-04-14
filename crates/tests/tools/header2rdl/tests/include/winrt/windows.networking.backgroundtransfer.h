
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
#ifndef __windows2Enetworking2Ebackgroundtransfer_h__
#define __windows2Enetworking2Ebackgroundtransfer_h__
#ifndef __windows2Enetworking2Ebackgroundtransfer_p_h__
#define __windows2Enetworking2Ebackgroundtransfer_p_h__


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
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
#include "Windows.UI.Notifications.h"
#include "Windows.Web.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloader;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloader

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloader2;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2 ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloader2

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloader3;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3 ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloader3

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloaderFactory;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloaderFactory

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloaderStaticMethods;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloaderStaticMethods

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloaderStaticMethods2;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2 ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloaderStaticMethods2

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundDownloaderUserConsent;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloaderUserConsent

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferBase;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferBase

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferCompletionGroup;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferCompletionGroup

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferCompletionGroupTriggerDetails;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferCompletionGroupTriggerDetails

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferContentPart;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferContentPart

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferContentPartFactory;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferContentPartFactory

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferErrorStaticMethods;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferErrorStaticMethods

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferGroup;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferGroupStatics;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroupStatics

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferOperation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferOperation

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferOperationPriority;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferOperationPriority

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundTransferRangesDownloadedEventArgs;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferRangesDownloadedEventArgs

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploader;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploader

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploader2;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2 ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploader2

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploader3;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3 ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploader3

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploaderFactory;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploaderFactory

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploaderStaticMethods;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploaderStaticMethods

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploaderStaticMethods2;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2 ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploaderStaticMethods2

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IBackgroundUploaderUserConsent;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploaderUserConsent

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IContentPrefetcher;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher ABI::Windows::Networking::BackgroundTransfer::IContentPrefetcher

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IContentPrefetcherTime;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime ABI::Windows::Networking::BackgroundTransfer::IContentPrefetcherTime

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IDownloadOperation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IDownloadOperation2;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2 ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation2

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IDownloadOperation3;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3 ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation3

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IDownloadOperation4;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4 ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation4

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IDownloadOperation5;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5 ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation5

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IResponseInformation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation ABI::Windows::Networking::BackgroundTransfer::IResponseInformation

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IUnconstrainedTransferRequestResult;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult ABI::Windows::Networking::BackgroundTransfer::IUnconstrainedTransferRequestResult

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IUploadOperation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation ABI::Windows::Networking::BackgroundTransfer::IUploadOperation

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IUploadOperation2;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2 ABI::Windows::Networking::BackgroundTransfer::IUploadOperation2

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IUploadOperation3;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3 ABI::Windows::Networking::BackgroundTransfer::IUploadOperation3

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                interface IUploadOperation4;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4 ABI::Windows::Networking::BackgroundTransfer::IUploadOperation4

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class DownloadOperation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2ab61055-2d0a-59cb-8cbd-056f2d7fb454"))
IIterator<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f6fd69cb-e6e7-56d5-9be6-e0dc4683fa80"))
IIterable<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f87d9755-2a7d-59fc-bc92-b48636f4d955"))
IVectorView<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ae42cddf-3042-5d92-a01e-643c252b8050"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.DownloadOperation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1986b372-0ddb-520c-b72d-fb2577e99ff5"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.DownloadOperation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class UploadOperation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("caa85133-73d7-5f96-ab2d-fbb4fa00f715"))
IIterator<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("79778799-38cc-5b67-9cd0-043fc47a9ef7"))
IIterable<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8e96d4b0-f0ae-51cb-b7c4-024251bd16d8"))
IVectorView<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e4c6a3c-48a4-5e22-b29a-3e429469462f"))
IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.UploadOperation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation*> __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("608a29a8-bbc5-5ea3-b3f7-87edc4e7bbbc"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.UploadOperation>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("43ef3a5f-cc7d-566d-a92a-4caa76b92a1f"))
IAsyncOperation<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2aa63857-ffaf-5cf6-9b2c-0dc597b60a60"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class UnconstrainedTransferRequestResult;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8dc2a04-3f44-5046-8182-cd0ec147e17d"))
IAsyncOperation<ABI::Windows::Networking::BackgroundTransfer::UnconstrainedTransferRequestResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UnconstrainedTransferRequestResult*, ABI::Windows::Networking::BackgroundTransfer::IUnconstrainedTransferRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::BackgroundTransfer::UnconstrainedTransferRequestResult*> __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4f52bfe8-9b0e-5b22-916b-834425b4ab97"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::UnconstrainedTransferRequestResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UnconstrainedTransferRequestResult*, ABI::Windows::Networking::BackgroundTransfer::IUnconstrainedTransferRequestResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::UnconstrainedTransferRequestResult*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44212ea1-b524-5aee-a320-7199225381d1"))
IAsyncOperation<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ead68818-0c38-5cde-aec1-c6a7618711f7"))
IAsyncOperationCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("781b479c-0207-5d15-a4e5-7837d13bf93d"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Networking.BackgroundTransfer.DownloadOperation, Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e86a4f5d-743a-5f18-9d4c-bc8ed5942659"))
IAsyncOperationWithProgress<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Networking.BackgroundTransfer.DownloadOperation, Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b2ff13f1-c743-54f4-bccc-f08e16a87890"))
IAsyncOperationProgressHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Networking.BackgroundTransfer.DownloadOperation, Windows.Networking.BackgroundTransfer.DownloadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*> __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("79fcae93-53ec-5f66-ab34-826af78ec11a"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Networking.BackgroundTransfer.UploadOperation, Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("35ddaefa-db6a-5d0d-ba54-a0728401171e"))
IAsyncOperationWithProgress<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Networking.BackgroundTransfer.UploadOperation, Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("ccd13730-fed3-54e8-8471-096e4b64cadd"))
IAsyncOperationProgressHandler<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::IUploadOperation*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Networking.BackgroundTransfer.UploadOperation, Windows.Networking.BackgroundTransfer.UploadOperation>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Networking::BackgroundTransfer::UploadOperation*, ABI::Windows::Networking::BackgroundTransfer::UploadOperation*> __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("60310303-49c5-52e6-abc6-a9b36eccc716"))
IKeyValuePair<HSTRING, HSTRING> : IKeyValuePair_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IKeyValuePair`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IKeyValuePair<HSTRING, HSTRING> __FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("05eb86f1-7140-5517-b88d-cbaebe57e6b1"))
IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterator_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */



#ifndef DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#define DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e9bdaaf0-cbf6-5c72-be90-29cbf3a1319b"))
IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> : IIterable_impl<__FIKeyValuePair_2_HSTRING_HSTRING*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<String, String>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<__FIKeyValuePair_2_HSTRING_HSTRING*> __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t;
#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_USE */


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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterator_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1c157d0f-5efe-5cec-bbd6-0c6ce9af07a5"))
IIterator<ABI::Windows::Foundation::Uri*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Foundation::Uri*> __FIIterator_1_Windows__CFoundation__CUri_t;
#define __FIIterator_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#define DEF___FIIterable_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b0d63b78-78ad-5e31-b6d8-e32a0e16c447"))
IIterable<ABI::Windows::Foundation::Uri*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Foundation::Uri*> __FIIterable_1_Windows__CFoundation__CUri_t;
#define __FIIterable_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class BackgroundTransferContentPart;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_USE
#define DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("07fbc351-781d-52c7-9558-a453e5703f29"))
IIterator<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferContentPart*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferContentPart*, ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferContentPart*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferContentPart*> __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_t;
#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_USE
#define DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("cf303199-de3b-5dac-a703-6c57d80821c4"))
IIterable<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferContentPart*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferContentPart*, ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferContentPart*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferContentPart*> __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_t;
#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef struct BackgroundTransferFileRange BackgroundTransferFileRange;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#define DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a753d778-8cbb-524a-b8c4-70c515a42782"))
IIterator<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> : IIterator_impl<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t;
#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#define DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2cc2d499-974c-5078-89ae-2d4ee1139721"))
IIterable<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> : IIterable_impl<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t;
#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace Web {
            typedef enum WebErrorStatus : int WebErrorStatus;
        } /* Web */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CWeb__CWebErrorStatus_USE
#define DEF___FIIterator_1_Windows__CWeb__CWebErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("fa704929-0761-5dd6-9675-052a8c61e2c2"))
IIterator<enum ABI::Windows::Web::WebErrorStatus> : IIterator_impl<enum ABI::Windows::Web::WebErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Web.WebErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Web::WebErrorStatus> __FIIterator_1_Windows__CWeb__CWebErrorStatus_t;
#define __FIIterator_1_Windows__CWeb__CWebErrorStatus ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CWeb__CWebErrorStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CWeb__CWebErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CWeb__CWebErrorStatus_USE
#define DEF___FIIterable_1_Windows__CWeb__CWebErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7b7f182e-a6ce-556b-9a2e-ef97662f2aee"))
IIterable<enum ABI::Windows::Web::WebErrorStatus> : IIterable_impl<enum ABI::Windows::Web::WebErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Web.WebErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Web::WebErrorStatus> __FIIterable_1_Windows__CWeb__CWebErrorStatus_t;
#define __FIIterable_1_Windows__CWeb__CWebErrorStatus ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CWeb__CWebErrorStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CWeb__CWebErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


#ifndef DEF___FIMapView_2_HSTRING_HSTRING_USE
#define DEF___FIMapView_2_HSTRING_HSTRING_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ac7f26f2-feb7-5b2a-8ac4-345bc62caede"))
IMapView<HSTRING, HSTRING> : IMapView_impl<HSTRING, HSTRING>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IMapView`2<String, String>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IMapView<HSTRING, HSTRING> __FIMapView_2_HSTRING_HSTRING_t;
#define __FIMapView_2_HSTRING_HSTRING ABI::Windows::Foundation::Collections::__FIMapView_2_HSTRING_HSTRING_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIMapView_2_HSTRING_HSTRING_USE */


#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#define DEF___FIVectorView_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4b8385bd-a2cd-5ff1-bf74-7ea580423e50"))
IVectorView<ABI::Windows::Foundation::Uri*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Foundation::Uri*> __FIVectorView_1_Windows__CFoundation__CUri_t;
#define __FIVectorView_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#define DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5be7934b-d9fc-540a-8ffe-5fb9c88c6558"))
IVectorView<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> : IVectorView_impl<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t;
#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CWeb__CWebErrorStatus_USE
#define DEF___FIVectorView_1_Windows__CWeb__CWebErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f5d10d42-a776-533a-8f4b-2e1c6e5bbf24"))
IVectorView<enum ABI::Windows::Web::WebErrorStatus> : IVectorView_impl<enum ABI::Windows::Web::WebErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Web.WebErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Web::WebErrorStatus> __FIVectorView_1_Windows__CWeb__CWebErrorStatus_t;
#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CWeb__CWebErrorStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CWeb__CWebErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CFoundation__CUri_USE
#define DEF___FIVector_1_Windows__CFoundation__CUri_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0d82bd8d-fe62-5d67-a7b9-7886dd75bc4e"))
IVector<ABI::Windows::Foundation::Uri*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Foundation::Uri*, ABI::Windows::Foundation::IUriRuntimeClass*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Foundation.Uri>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Foundation::Uri*> __FIVector_1_Windows__CFoundation__CUri_t;
#define __FIVector_1_Windows__CFoundation__CUri ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CFoundation__CUri_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CFoundation__CUri_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#define DEF___FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c73ceef0-854a-5947-9e7c-527e3915d335"))
IVector<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> : IVector_impl<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<struct ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferFileRange> __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t;
#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CWeb__CWebErrorStatus_USE
#define DEF___FIVector_1_Windows__CWeb__CWebErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("61bc06e3-b752-5b56-8374-3b45a214693f"))
IVector<enum ABI::Windows::Web::WebErrorStatus> : IVector_impl<enum ABI::Windows::Web::WebErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Web.WebErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<enum ABI::Windows::Web::WebErrorStatus> __FIVector_1_Windows__CWeb__CWebErrorStatus_t;
#define __FIVector_1_Windows__CWeb__CWebErrorStatus ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CWeb__CWebErrorStatus_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CWeb__CWebErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIReference_1_Windows__CWeb__CWebErrorStatus_USE
#define DEF___FIReference_1_Windows__CWeb__CWebErrorStatus_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f2b26336-6a9d-54de-8eca-00d6c871e469"))
IReference<enum ABI::Windows::Web::WebErrorStatus> : IReference_impl<enum ABI::Windows::Web::WebErrorStatus>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Web.WebErrorStatus>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::Web::WebErrorStatus> __FIReference_1_Windows__CWeb__CWebErrorStatus_t;
#define __FIReference_1_Windows__CWeb__CWebErrorStatus ABI::Windows::Foundation::__FIReference_1_Windows__CWeb__CWebErrorStatus_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CWeb__CWebErrorStatus_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class BackgroundTransferRangesDownloadedEventArgs;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("93a20d85-bdfc-5195-90d9-8cb56cbcb3d8"))
ITypedEventHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferRangesDownloadedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferRangesDownloadedEventArgs*, ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferRangesDownloadedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Networking.BackgroundTransfer.DownloadOperation, Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Networking::BackgroundTransfer::DownloadOperation*, ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferRangesDownloadedEventArgs*> __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Background {
                interface IBackgroundTrigger;
            } /* Background */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger ABI::Windows::ApplicationModel::Background::IBackgroundTrigger

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__

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
        namespace UI {
            namespace Notifications {
                class TileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface ITileNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CITileNotification ABI::Windows::UI::Notifications::ITileNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                class ToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace UI {
            namespace Notifications {
                interface IToastNotification;
            } /* Notifications */
        } /* UI */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CUI_CNotifications_CIToastNotification ABI::Windows::UI::Notifications::IToastNotification

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef enum BackgroundTransferBehavior : int BackgroundTransferBehavior;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef enum BackgroundTransferCostPolicy : int BackgroundTransferCostPolicy;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef enum BackgroundTransferPriority : int BackgroundTransferPriority;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef enum BackgroundTransferStatus : int BackgroundTransferStatus;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef struct BackgroundDownloadProgress BackgroundDownloadProgress;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                typedef struct BackgroundUploadProgress BackgroundUploadProgress;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class BackgroundDownloader;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class BackgroundTransferCompletionGroup;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class BackgroundTransferGroup;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class BackgroundUploader;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                class ResponseInformation;
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                enum BackgroundTransferBehavior : int
                {
                    BackgroundTransferBehavior_Parallel = 0,
                    BackgroundTransferBehavior_Serialized = 1,
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferCostPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                enum BackgroundTransferCostPolicy : int
                {
                    BackgroundTransferCostPolicy_Default = 0,
                    BackgroundTransferCostPolicy_UnrestrictedOnly = 1,
                    BackgroundTransferCostPolicy_Always = 2,
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                enum BackgroundTransferPriority : int
                {
                    BackgroundTransferPriority_Default = 0,
                    BackgroundTransferPriority_High = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                    BackgroundTransferPriority_Low = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                enum BackgroundTransferStatus : int
                {
                    BackgroundTransferStatus_Idle = 0,
                    BackgroundTransferStatus_Running = 1,
                    BackgroundTransferStatus_PausedByApplication = 2,
                    BackgroundTransferStatus_PausedCostedNetwork = 3,
                    BackgroundTransferStatus_PausedNoNetwork = 4,
                    BackgroundTransferStatus_Completed = 5,
                    BackgroundTransferStatus_Canceled = 6,
                    BackgroundTransferStatus_Error = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    BackgroundTransferStatus_PausedRecoverableWebErrorStatus = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    BackgroundTransferStatus_PausedSystemPolicy = 32,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundDownloadProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                struct BackgroundDownloadProgress
                {
                    UINT64 BytesReceived;
                    UINT64 TotalBytesToReceive;
                    ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferStatus Status;
                    boolean HasResponseChanged;
                    boolean HasRestarted;
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                struct BackgroundTransferFileRange
                {
                    UINT64 Offset;
                    UINT64 Length;
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundUploadProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                struct BackgroundUploadProgress
                {
                    UINT64 BytesReceived;
                    UINT64 BytesSent;
                    UINT64 TotalBytesToReceive;
                    UINT64 TotalBytesToSend;
                    ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferStatus Status;
                    boolean HasResponseChanged;
                    boolean HasRestarted;
                };
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloader[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloader";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("c1c79333-6649-4b1d-a826-a4b3dd234d0b")
                IBackgroundDownloader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateDownload(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Storage::IStorageFile* resultFile,
                        ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDownloadFromFile(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Storage::IStorageFile* resultFile,
                        ABI::Windows::Storage::IStorageFile* requestBodyFile,
                        ABI::Windows::Networking::BackgroundTransfer::IDownloadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateDownloadAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Storage::IStorageFile* resultFile,
                        ABI::Windows::Storage::Streams::IInputStream* requestBodyStream,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloader = __uuidof(IBackgroundDownloader);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloader2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloader2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("a94a5847-348d-4a35-890e-8a1ef3798479")
                IBackgroundDownloader2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransferGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransferGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuccessToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuccessToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FailureToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FailureToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuccessTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuccessTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FailureTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FailureTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloader2 = __uuidof(IBackgroundDownloader2);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloader3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloader3[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloader3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("d11a8c48-86e8-48e2-b615-6976aabf861d")
                IBackgroundDownloader3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CompletionGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferCompletionGroup** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloader3 = __uuidof(IBackgroundDownloader3);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderFactory[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("26836c24-d89e-46f4-a29a-4f4d4f144155")
                IBackgroundDownloaderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithCompletionGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferCompletionGroup* completionGroup,
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundDownloader** backgroundDownloader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloaderFactory = __uuidof(IBackgroundDownloaderFactory);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderStaticMethods[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("52a65a35-c64e-426c-9919-540d0d21a650")
                IBackgroundDownloaderStaticMethods : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentDownloadsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("GetCurrentDownloadsAsync(group) may be altered or unavailable for releases after Windows 8.1. Instead, use GetCurrentDownloadsForTransferGroupAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentDownloadsForGroupAsync(
                        HSTRING group,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloaderStaticMethods = __uuidof(IBackgroundDownloaderStaticMethods);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderStaticMethods2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("2faa1327-1ad4-4ca5-b2cd-08dbf0746afe")
                IBackgroundDownloaderStaticMethods2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentDownloadsForTransferGroupAsync(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup* group,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloaderStaticMethods2 = __uuidof(IBackgroundDownloaderStaticMethods2);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderUserConsent[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("5d14e906-9266-4808-bd71-5925f2a3130a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                DEPRECATED("IBackgroundDownloaderUserConsent is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                IBackgroundDownloaderUserConsent : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("RequestUnconstrainedDownloadsAsync is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    virtual HRESULT STDMETHODCALLTYPE RequestUnconstrainedDownloadsAsync(
                        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* operations,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundDownloaderUserConsent = __uuidof(IBackgroundDownloaderUserConsent);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferBase[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferBase";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("2a9da250-c769-458c-afe8-feb8d4d3b2ef")
                IBackgroundTransferBase : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetRequestHeader(
                        HSTRING headerName,
                        HSTRING headerValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ServerCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ServerCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProxyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential** credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProxyCredential(
                        ABI::Windows::Security::Credentials::IPasswordCredential* credential
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Method(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Method(
                        HSTRING value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Group(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE put_Group(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CostPolicy(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferCostPolicy* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CostPolicy(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferCostPolicy value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferBase = __uuidof(IBackgroundTransferBase);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferCompletionGroup[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("2d930225-986b-574d-7950-0add47f5d706")
                IBackgroundTransferCompletionGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Trigger(
                        ABI::Windows::ApplicationModel::Background::IBackgroundTrigger** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEnabled(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Enable(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferCompletionGroup = __uuidof(IBackgroundTransferCompletionGroup);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferCompletionGroupTriggerDetails[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("7b6be286-6e47-5136-7fcb-fa4389f46f5b")
                IBackgroundTransferCompletionGroupTriggerDetails : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Downloads(
                        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Uploads(
                        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferCompletionGroupTriggerDetails = __uuidof(IBackgroundTransferCompletionGroupTriggerDetails);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferContentPart[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("e8e15657-d7d1-4ed8-838e-674ac217ace6")
                IBackgroundTransferContentPart : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetHeader(
                        HSTRING headerName,
                        HSTRING headerValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetText(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetFile(
                        ABI::Windows::Storage::IStorageFile* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferContentPart = __uuidof(IBackgroundTransferContentPart);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferContentPartFactory[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("90ef98a9-7a01-4a0b-9f80-a0b0bb370f8d")
                IBackgroundTransferContentPartFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithName(
                        HSTRING name,
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferContentPart** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWithNameAndFileName(
                        HSTRING name,
                        HSTRING fileName,
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferContentPart** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferContentPartFactory = __uuidof(IBackgroundTransferContentPartFactory);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferErrorStaticMethods[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("aad33b04-1192-4bf4-8b68-39c5add244e2")
                IBackgroundTransferErrorStaticMethods : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetStatus(
                        INT32 hresult,
                        ABI::Windows::Web::WebErrorStatus* status
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferErrorStaticMethods = __uuidof(IBackgroundTransferErrorStaticMethods);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferGroup[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("d8c3e3e4-6459-4540-85eb-aaa1c8903677")
                IBackgroundTransferGroup : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Name(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TransferBehavior(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferBehavior* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransferBehavior(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferBehavior value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferGroup = __uuidof(IBackgroundTransferGroup);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferGroupStatics[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("02ec50b2-7d18-495b-aa22-32a97d45d3e2")
                IBackgroundTransferGroupStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateGroup(
                        HSTRING name,
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferGroupStatics = __uuidof(IBackgroundTransferGroupStatics);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferOperation[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("ded06846-90ca-44fb-8fb1-124154c0d539")
                IBackgroundTransferOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Guid(
                        GUID* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RequestedUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Method(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE get_Group(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CostPolicy(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferCostPolicy* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_CostPolicy(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferCostPolicy value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetResultStreamAt(
                        UINT64 position,
                        ABI::Windows::Storage::Streams::IInputStream** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetResponseInformation(
                        ABI::Windows::Networking::BackgroundTransfer::IResponseInformation** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferOperation = __uuidof(IBackgroundTransferOperation);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferOperationPriority[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("04854327-5254-4b3a-915e-0aa49275c0f9")
                IBackgroundTransferOperationPriority : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Priority(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferPriority* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Priority(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundTransferPriority value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferOperationPriority = __uuidof(IBackgroundTransferOperationPriority);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferRangesDownloadedEventArgs[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("3ebc7453-bf48-4a88-9248-b0c165184f5c")
                IBackgroundTransferRangesDownloadedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_WasDownloadRestarted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AddedRanges(
                        __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                        ABI::Windows::Foundation::IDeferral** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundTransferRangesDownloadedEventArgs = __uuidof(IBackgroundTransferRangesDownloadedEventArgs);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploader[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploader";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("c595c9ae-cead-465b-8801-c55ac90a01ce")
                IBackgroundUploader : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateUpload(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Storage::IStorageFile* sourceFile,
                        ABI::Windows::Networking::BackgroundTransfer::IUploadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateUploadFromStreamAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        ABI::Windows::Storage::Streams::IInputStream* sourceStream,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateUploadWithFormDataAndAutoBoundaryAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* parts,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateUploadWithSubTypeAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* parts,
                        HSTRING subType,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateUploadWithSubTypeAndBoundaryAsync(
                        ABI::Windows::Foundation::IUriRuntimeClass* uri,
                        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* parts,
                        HSTRING subType,
                        HSTRING boundary,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploader = __uuidof(IBackgroundUploader);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploader2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploader2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("8e0612ce-0c34-4463-807f-198a1b8bd4ad")
                IBackgroundUploader2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransferGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_TransferGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuccessToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuccessToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FailureToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FailureToastNotification(
                        ABI::Windows::UI::Notifications::IToastNotification* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SuccessTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SuccessTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FailureTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_FailureTileNotification(
                        ABI::Windows::UI::Notifications::ITileNotification* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploader2 = __uuidof(IBackgroundUploader2);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploader3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploader3[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploader3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("b95e9439-5bf0-4b3a-8c47-2c6199a854b9")
                IBackgroundUploader3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CompletionGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferCompletionGroup** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploader3 = __uuidof(IBackgroundUploader3);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderFactory[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("736203c7-10e7-48a0-ac3c-1ac71095ec57")
                IBackgroundUploaderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWithCompletionGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferCompletionGroup* completionGroup,
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundUploader** backgroundUploader
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploaderFactory = __uuidof(IBackgroundUploaderFactory);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderStaticMethods[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("f2875cfb-9b05-4741-9121-740a83e247df")
                IBackgroundUploaderStaticMethods : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentUploadsAsync(
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    DEPRECATED("GetCurrentUploadsAsync(group) may be altered or unavailable for releases after Windows 8.1. Instead, use GetCurrentUploadsForTransferGroupAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentUploadsForGroupAsync(
                        HSTRING group,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploaderStaticMethods = __uuidof(IBackgroundUploaderStaticMethods);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderStaticMethods2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("e919ac62-ea08-42f0-a2ac-07e467549080")
                IBackgroundUploaderStaticMethods2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentUploadsForTransferGroupAsync(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup* group,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploaderStaticMethods2 = __uuidof(IBackgroundUploaderStaticMethods2);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderUserConsent[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("3bb384cb-0760-461d-907f-5138f84d44c1")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                DEPRECATED("IBackgroundUploaderUserConsent is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                IBackgroundUploaderUserConsent : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("RequestUnconstrainedUploadsAsync is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    virtual HRESULT STDMETHODCALLTYPE RequestUnconstrainedUploadsAsync(
                        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* operations,
                        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IBackgroundUploaderUserConsent = __uuidof(IBackgroundUploaderUserConsent);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IContentPrefetcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.ContentPrefetcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IContentPrefetcher[] = L"Windows.Networking.BackgroundTransfer.IContentPrefetcher";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("a8d6f754-7dc1-4cd9-8810-2a6aa9417e11")
                IContentPrefetcher : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ContentUris(
                        __FIVector_1_Windows__CFoundation__CUri** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IndirectContentUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IndirectContentUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContentPrefetcher = __uuidof(IContentPrefetcher);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IContentPrefetcherTime
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.ContentPrefetcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IContentPrefetcherTime[] = L"Windows.Networking.BackgroundTransfer.IContentPrefetcherTime";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("e361fd08-132a-4fde-a7cc-fcb0e66523af")
                IContentPrefetcherTime : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LastSuccessfulPrefetchTime(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContentPrefetcherTime = __uuidof(IContentPrefetcherTime);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("bd87ebb0-5714-4e09-ba68-bef73903b0d7")
                IDownloadOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ResultFile(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Progress(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundDownloadProgress* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AttachAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Resume(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IDownloadOperation = __uuidof(IDownloadOperation);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation2[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("a3cced40-8f9c-4353-9cd4-290dee387c38")
                IDownloadOperation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransferGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDownloadOperation2 = __uuidof(IDownloadOperation2);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation3[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("5027351c-7d5e-4adc-b8d3-df5c6031b9cc")
                IDownloadOperation3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsRandomAccessRequired(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsRandomAccessRequired(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetResultRandomAccessStreamReference(
                        ABI::Windows::Storage::Streams::IRandomAccessStreamReference** stream
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetDownloadedRanges(
                        __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_RangesDownloaded(
                        __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs* eventHandler,
                        EventRegistrationToken* eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_RangesDownloaded(
                        EventRegistrationToken eventCookie
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_RequestedUri(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RecoverableWebErrorStatuses(
                        __FIVector_1_Windows__CWeb__CWebErrorStatus** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentWebErrorStatus(
                        __FIReference_1_Windows__CWeb__CWebErrorStatus** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDownloadOperation3 = __uuidof(IDownloadOperation3);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation4[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation4";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("0cdaaef4-8cef-404a-966d-f058400bed80")
                IDownloadOperation4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE MakeCurrentInTransferGroup(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IDownloadOperation4 = __uuidof(IDownloadOperation4);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation5[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation5";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("a699a86f-5590-463a-b8d6-1e491a2760a5")
                IDownloadOperation5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetRequestHeader(
                        HSTRING headerName,
                        HSTRING headerValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveRequestHeader(
                        HSTRING headerName
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IDownloadOperation5 = __uuidof(IDownloadOperation5);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IResponseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.ResponseInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IResponseInformation[] = L"Windows.Networking.BackgroundTransfer.IResponseInformation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("f8bb9a12-f713-4792-8b68-d9d297f91d2e")
                IResponseInformation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsResumable(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActualUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StatusCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Headers(
                        __FIMapView_2_HSTRING_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IResponseInformation = __uuidof(IResponseInformation);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUnconstrainedTransferRequestResult[] = L"Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("4c24b81f-d944-4112-a98e-6a69522b7ebb")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                DEPRECATED("IUnconstrainedTransferRequestResult is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                IUnconstrainedTransferRequestResult : public IInspectable
                {
                public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("IsUnconstrained is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    virtual HRESULT STDMETHODCALLTYPE get_IsUnconstrained(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUnconstrainedTransferRequestResult = __uuidof(IUnconstrainedTransferRequestResult);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("3e5624e0-7389-434c-8b35-427fd36bbdae")
                IUploadOperation : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SourceFile(
                        ABI::Windows::Storage::IStorageFile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Progress(
                        ABI::Windows::Networking::BackgroundTransfer::BackgroundUploadProgress* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AttachAsync(
                        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUploadOperation = __uuidof(IUploadOperation);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation2[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation2";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("556189f2-2774-4df6-9fa5-209f2bfb12f7")
                IUploadOperation2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_TransferGroup(
                        ABI::Windows::Networking::BackgroundTransfer::IBackgroundTransferGroup** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUploadOperation2 = __uuidof(IUploadOperation2);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation3[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation3";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("42c92ca3-de39-4546-bc62-3774b4294de3")
                IUploadOperation3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE MakeCurrentInTransferGroup(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IUploadOperation3 = __uuidof(IUploadOperation3);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation4[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation4";
namespace ABI {
    namespace Windows {
        namespace Networking {
            namespace BackgroundTransfer {
                MIDL_INTERFACE("50edef31-fac5-41ee-b030-dc77caee9faa")
                IUploadOperation4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetRequestHeader(
                        HSTRING headerName,
                        HSTRING headerValue
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RemoveRequestHeader(
                        HSTRING headerName
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IUploadOperation4 = __uuidof(IUploadOperation4);
            } /* BackgroundTransfer */
        } /* Networking */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundDownloader ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *    Windows.Networking.BackgroundTransfer.IBackgroundDownloader2
 *    Windows.Networking.BackgroundTransfer.IBackgroundDownloader3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundDownloader_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundDownloader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundDownloader[] = L"Windows.Networking.BackgroundTransfer.BackgroundDownloader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroup_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroup[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroupTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroupTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroupTriggerDetails[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferContentPart_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferContentPart_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferContentPart[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferError_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferError[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferGroup_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferGroup[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferRangesDownloadedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferRangesDownloadedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferRangesDownloadedEventArgs[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundUploader ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *    Windows.Networking.BackgroundTransfer.IBackgroundUploader2
 *    Windows.Networking.BackgroundTransfer.IBackgroundUploader3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundUploader_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundUploader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundUploader[] = L"Windows.Networking.BackgroundTransfer.BackgroundUploader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.ContentPrefetcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IContentPrefetcher interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IContentPrefetcherTime interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ContentPrefetcher_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ContentPrefetcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_ContentPrefetcher[] = L"Windows.Networking.BackgroundTransfer.ContentPrefetcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation2
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation3
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation4
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_DownloadOperation_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_DownloadOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_DownloadOperation[] = L"Windows.Networking.BackgroundTransfer.DownloadOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.ResponseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IResponseInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ResponseInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ResponseInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_ResponseInformation[] = L"Windows.Networking.BackgroundTransfer.ResponseInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UnconstrainedTransferRequestResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UnconstrainedTransferRequestResult_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("UnconstrainedTransferRequestResult is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_UnconstrainedTransferRequestResult[] = L"Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.UploadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IUploadOperation ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority
 *    Windows.Networking.BackgroundTransfer.IUploadOperation2
 *    Windows.Networking.BackgroundTransfer.IUploadOperation3
 *    Windows.Networking.BackgroundTransfer.IUploadOperation4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UploadOperation_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UploadOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_UploadOperation[] = L"Windows.Networking.BackgroundTransfer.UploadOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_FWD_DEFINED__
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4 __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4;

#endif // ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        UINT32 index,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResultVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* This,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* asyncInfo,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* asyncInfo,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if !defined(____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIKeyValuePair_2_HSTRING_HSTRING __FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* result);

    END_INTERFACE
} __FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Key(This, result) \
    ((This)->lpVtbl->get_Key(This, result))

#define __FIKeyValuePair_2_HSTRING_HSTRING_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIKeyValuePair_2_HSTRING_HSTRING** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        UINT32 itemsLength,
        __FIKeyValuePair_2_HSTRING_HSTRING** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if !defined(____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING;

typedef struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING* This,
        __FIIterator_1___FIKeyValuePair_2_HSTRING_HSTRING** result);

    END_INTERFACE
} __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl;

interface __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1___FIKeyValuePair_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CFoundation__CUri __FIIterator_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CFoundation__CUri;

typedef struct __FIIterator_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CFoundation__CUri* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CFoundation__CUriVtbl;

interface __FIIterator_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterator_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CFoundation__CUri_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CFoundation__CUri_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CFoundation__CUri __FIIterable_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CFoundation__CUri;

typedef struct __FIIterable_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CFoundation__CUri* This,
        __FIIterator_1_Windows__CFoundation__CUri** result);

    END_INTERFACE
} __FIIterable_1_Windows__CFoundation__CUriVtbl;

interface __FIIterable_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIIterable_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CFoundation__CUri_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart;

typedef struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPartVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPartVtbl;

interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPartVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart;

typedef struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPartVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* This,
        __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPartVtbl;

interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPartVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

typedef struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl;

interface __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange
{
    CONST_VTBL struct __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

typedef struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        __FIIterator_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange** result);

    END_INTERFACE
} __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl;

interface __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange
{
    CONST_VTBL struct __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

typedef enum __x_ABI_CWindows_CWeb_CWebErrorStatus __x_ABI_CWindows_CWeb_CWebErrorStatus;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CWeb__CWebErrorStatus __FIIterator_1_Windows__CWeb__CWebErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CWeb__CWebErrorStatus;

typedef struct __FIIterator_1_Windows__CWeb__CWebErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CWeb__CWebErrorStatusVtbl;

interface __FIIterator_1_Windows__CWeb__CWebErrorStatus
{
    CONST_VTBL struct __FIIterator_1_Windows__CWeb__CWebErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CWeb__CWebErrorStatus_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CWeb__CWebErrorStatus __FIIterable_1_Windows__CWeb__CWebErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CWeb__CWebErrorStatus;

typedef struct __FIIterable_1_Windows__CWeb__CWebErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CWeb__CWebErrorStatus* This,
        __FIIterator_1_Windows__CWeb__CWebErrorStatus** result);

    END_INTERFACE
} __FIIterable_1_Windows__CWeb__CWebErrorStatusVtbl;

interface __FIIterable_1_Windows__CWeb__CWebErrorStatus
{
    CONST_VTBL struct __FIIterable_1_Windows__CWeb__CWebErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CWeb__CWebErrorStatus_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

#if !defined(____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__)
#define ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

typedef interface __FIMapView_2_HSTRING_HSTRING __FIMapView_2_HSTRING_HSTRING;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIMapView_2_HSTRING_HSTRING;

typedef struct __FIMapView_2_HSTRING_HSTRINGVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIMapView_2_HSTRING_HSTRING* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIMapView_2_HSTRING_HSTRING* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIMapView_2_HSTRING_HSTRING* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIMapView_2_HSTRING_HSTRING* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIMapView_2_HSTRING_HSTRING* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Lookup)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIMapView_2_HSTRING_HSTRING* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* HasKey)(__FIMapView_2_HSTRING_HSTRING* This,
        HSTRING key,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* Split)(__FIMapView_2_HSTRING_HSTRING* This,
        __FIMapView_2_HSTRING_HSTRING** first,
        __FIMapView_2_HSTRING_HSTRING** second);

    END_INTERFACE
} __FIMapView_2_HSTRING_HSTRINGVtbl;

interface __FIMapView_2_HSTRING_HSTRING
{
    CONST_VTBL struct __FIMapView_2_HSTRING_HSTRINGVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIMapView_2_HSTRING_HSTRING_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIMapView_2_HSTRING_HSTRING_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIMapView_2_HSTRING_HSTRING_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIMapView_2_HSTRING_HSTRING_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIMapView_2_HSTRING_HSTRING_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIMapView_2_HSTRING_HSTRING_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIMapView_2_HSTRING_HSTRING_Lookup(This, key, result) \
    ((This)->lpVtbl->Lookup(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIMapView_2_HSTRING_HSTRING_HasKey(This, key, result) \
    ((This)->lpVtbl->HasKey(This, key, result))

#define __FIMapView_2_HSTRING_HSTRING_Split(This, first, second) \
    ((This)->lpVtbl->Split(This, first, second))

#endif /* COBJMACROS */

#endif // ____FIMapView_2_HSTRING_HSTRING_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CFoundation__CUri __FIVectorView_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CFoundation__CUri;

typedef struct __FIVectorView_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CFoundation__CUri* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CFoundation__CUriVtbl;

interface __FIVectorView_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIVectorView_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CFoundation__CUri_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

typedef struct __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl;

interface __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange
{
    CONST_VTBL struct __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CWeb__CWebErrorStatus __FIVectorView_1_Windows__CWeb__CWebErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CWeb__CWebErrorStatus;

typedef struct __FIVectorView_1_Windows__CWeb__CWebErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 index,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CWeb__CWebErrorStatusVtbl;

interface __FIVectorView_1_Windows__CWeb__CWebErrorStatus
{
    CONST_VTBL struct __FIVectorView_1_Windows__CWeb__CWebErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CWeb__CWebErrorStatus_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CFoundation__CUri __FIVector_1_Windows__CFoundation__CUri;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CFoundation__CUri;

typedef struct __FIVector_1_Windows__CFoundation__CUriVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CFoundation__CUri* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CFoundation__CUri* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CFoundation__CUri* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CFoundation__CUri* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CFoundation__CUri* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CFoundation__CUri* This,
        __FIVectorView_1_Windows__CFoundation__CUri** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CFoundation__CUri* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CFoundation__CUri* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CFoundation__CUri* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** items);

    END_INTERFACE
} __FIVector_1_Windows__CFoundation__CUriVtbl;

interface __FIVector_1_Windows__CFoundation__CUri
{
    CONST_VTBL struct __FIVector_1_Windows__CFoundation__CUriVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CFoundation__CUri_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CFoundation__CUri_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CFoundation__CUri_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CFoundation__CUri_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CFoundation__CUri_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CFoundation__CUri_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CFoundation__CUri_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CFoundation__CUri_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CFoundation__CUri_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CFoundation__CUri_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CFoundation__CUri_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CFoundation__CUri_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CFoundation__CUri_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CFoundation__CUri_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CFoundation__CUri_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CFoundation__CUri_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CFoundation__CUri_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CFoundation__CUri_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CFoundation__CUri_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange;

typedef struct __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 index,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange* This,
        UINT32 itemsLength,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange* items);

    END_INTERFACE
} __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl;

interface __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange
{
    CONST_VTBL struct __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRangeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CWeb__CWebErrorStatus __FIVector_1_Windows__CWeb__CWebErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CWeb__CWebErrorStatus;

typedef struct __FIVector_1_Windows__CWeb__CWebErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 index,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        __FIVectorView_1_Windows__CWeb__CWebErrorStatus** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 index,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 index,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CWeb__CWebErrorStatus* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* items);

    END_INTERFACE
} __FIVector_1_Windows__CWeb__CWebErrorStatusVtbl;

interface __FIVector_1_Windows__CWeb__CWebErrorStatus
{
    CONST_VTBL struct __FIVector_1_Windows__CWeb__CWebErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CWeb__CWebErrorStatus_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIReference_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CWeb__CWebErrorStatus __FIReference_1_Windows__CWeb__CWebErrorStatus;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CWeb__CWebErrorStatus;

typedef struct __FIReference_1_Windows__CWeb__CWebErrorStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CWeb__CWebErrorStatus* This,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* result);

    END_INTERFACE
} __FIReference_1_Windows__CWeb__CWebErrorStatusVtbl;

interface __FIReference_1_Windows__CWeb__CWebErrorStatus
{
    CONST_VTBL struct __FIReference_1_Windows__CWeb__CWebErrorStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CWeb__CWebErrorStatus_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CWeb__CWebErrorStatus_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* sender,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger;

#endif // ____x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential;

#endif // ____x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIInputStream __x_ABI_CWindows_CStorage_CStreams_CIInputStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIInputStream_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CITileNotification __x_ABI_CWindows_CUI_CNotifications_CITileNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CITileNotification_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
#define ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CUI_CNotifications_CIToastNotification __x_ABI_CWindows_CUI_CNotifications_CIToastNotification;

#endif // ____x_ABI_CWindows_CUI_CNotifications_CIToastNotification_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferBehavior __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferBehavior;

typedef enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy;

typedef enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferPriority __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferPriority;

typedef enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferStatus __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferStatus;

typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundDownloadProgress __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundDownloadProgress;

typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundUploadProgress __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundUploadProgress;

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferBehavior
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferBehavior
{
    BackgroundTransferBehavior_Parallel = 0,
    BackgroundTransferBehavior_Serialized = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferCostPolicy
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy
{
    BackgroundTransferCostPolicy_Default = 0,
    BackgroundTransferCostPolicy_UnrestrictedOnly = 1,
    BackgroundTransferCostPolicy_Always = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferPriority
{
    BackgroundTransferPriority_Default = 0,
    BackgroundTransferPriority_High = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
    BackgroundTransferPriority_Low = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferStatus
{
    BackgroundTransferStatus_Idle = 0,
    BackgroundTransferStatus_Running = 1,
    BackgroundTransferStatus_PausedByApplication = 2,
    BackgroundTransferStatus_PausedCostedNetwork = 3,
    BackgroundTransferStatus_PausedNoNetwork = 4,
    BackgroundTransferStatus_Completed = 5,
    BackgroundTransferStatus_Canceled = 6,
    BackgroundTransferStatus_Error = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    BackgroundTransferStatus_PausedRecoverableWebErrorStatus = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    BackgroundTransferStatus_PausedSystemPolicy = 32,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundDownloadProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundDownloadProgress
{
    UINT64 BytesReceived;
    UINT64 TotalBytesToReceive;
    enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferStatus Status;
    boolean HasResponseChanged;
    boolean HasRestarted;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundTransferFileRange
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferFileRange
{
    UINT64 Offset;
    UINT64 Length;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Struct Windows.Networking.BackgroundTransfer.BackgroundUploadProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundUploadProgress
{
    UINT64 BytesReceived;
    UINT64 BytesSent;
    UINT64 TotalBytesToReceive;
    UINT64 TotalBytesToSend;
    enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferStatus Status;
    boolean HasResponseChanged;
    boolean HasRestarted;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloader[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloader";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateDownload)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CIStorageFile* resultFile,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* CreateDownloadFromFile)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CIStorageFile* resultFile,
        __x_ABI_CWindows_CStorage_CIStorageFile* requestBodyFile,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* CreateDownloadAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CIStorageFile* resultFile,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* requestBodyStream,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_CreateDownload(This, uri, resultFile, operation) \
    ((This)->lpVtbl->CreateDownload(This, uri, resultFile, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_CreateDownloadFromFile(This, uri, resultFile, requestBodyFile, operation) \
    ((This)->lpVtbl->CreateDownloadFromFile(This, uri, resultFile, requestBodyFile, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_CreateDownloadAsync(This, uri, resultFile, requestBodyStream, operation) \
    ((This)->lpVtbl->CreateDownloadAsync(This, uri, resultFile, requestBodyStream, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloader2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloader2";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup** value);
    HRESULT (STDMETHODCALLTYPE* put_TransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* value);
    HRESULT (STDMETHODCALLTYPE* get_SuccessToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_SuccessToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* value);
    HRESULT (STDMETHODCALLTYPE* get_FailureToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_FailureToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* value);
    HRESULT (STDMETHODCALLTYPE* get_SuccessTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_SuccessTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification* value);
    HRESULT (STDMETHODCALLTYPE* get_FailureTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_FailureTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_get_TransferGroup(This, value) \
    ((This)->lpVtbl->get_TransferGroup(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_put_TransferGroup(This, value) \
    ((This)->lpVtbl->put_TransferGroup(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_get_SuccessToastNotification(This, value) \
    ((This)->lpVtbl->get_SuccessToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_put_SuccessToastNotification(This, value) \
    ((This)->lpVtbl->put_SuccessToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_get_FailureToastNotification(This, value) \
    ((This)->lpVtbl->get_FailureToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_put_FailureToastNotification(This, value) \
    ((This)->lpVtbl->put_FailureToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_get_SuccessTileNotification(This, value) \
    ((This)->lpVtbl->get_SuccessTileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_put_SuccessTileNotification(This, value) \
    ((This)->lpVtbl->put_SuccessTileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_get_FailureTileNotification(This, value) \
    ((This)->lpVtbl->get_FailureTileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_put_FailureTileNotification(This, value) \
    ((This)->lpVtbl->put_FailureTileNotification(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloader3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloader3[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloader3";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CompletionGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_get_CompletionGroup(This, value) \
    ((This)->lpVtbl->get_CompletionGroup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderFactory[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithCompletionGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* completionGroup,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloader** backgroundDownloader);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_CreateWithCompletionGroup(This, completionGroup, backgroundDownloader) \
    ((This)->lpVtbl->CreateWithCompletionGroup(This, completionGroup, backgroundDownloader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderStaticMethods[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethodsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentDownloadsAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetCurrentDownloadsAsync(group) may be altered or unavailable for releases after Windows 8.1. Instead, use GetCurrentDownloadsForTransferGroupAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetCurrentDownloadsForGroupAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods* This,
        HSTRING group,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethodsVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethodsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_GetCurrentDownloadsAsync(This, operation) \
    ((This)->lpVtbl->GetCurrentDownloadsAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetCurrentDownloadsAsync(group) may be altered or unavailable for releases after Windows 8.1. Instead, use GetCurrentDownloadsForTransferGroupAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_GetCurrentDownloadsForGroupAsync(This, group, operation) \
    ((This)->lpVtbl->GetCurrentDownloadsForGroupAsync(This, group, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderStaticMethods2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentDownloadsForTransferGroupAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* group,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_GetCurrentDownloadsForTransferGroupAsync(This, group, operation) \
    ((This)->lpVtbl->GetCurrentDownloadsForTransferGroupAsync(This, group, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderStaticMethods2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundDownloaderUserConsent[] = L"Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("IBackgroundDownloaderUserConsent is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("RequestUnconstrainedDownloadsAsync is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* RequestUnconstrainedDownloadsAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent* This,
        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation* operations,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsentVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("RequestUnconstrainedDownloadsAsync is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_RequestUnconstrainedDownloadsAsync(This, operations, operation) \
    ((This)->lpVtbl->RequestUnconstrainedDownloadsAsync(This, operations, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundDownloaderUserConsent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferBase[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferBase";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBaseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetRequestHeader)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        HSTRING headerName,
        HSTRING headerValue);
    HRESULT (STDMETHODCALLTYPE* get_ServerCredential)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** credential);
    HRESULT (STDMETHODCALLTYPE* put_ServerCredential)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* credential);
    HRESULT (STDMETHODCALLTYPE* get_ProxyCredential)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential** credential);
    HRESULT (STDMETHODCALLTYPE* put_ProxyCredential)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        __x_ABI_CWindows_CSecurity_CCredentials_CIPasswordCredential* credential);
    HRESULT (STDMETHODCALLTYPE* get_Method)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Method)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        HSTRING value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Group)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* put_Group)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_CostPolicy)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy* value);
    HRESULT (STDMETHODCALLTYPE* put_CostPolicy)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBaseVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBaseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_SetRequestHeader(This, headerName, headerValue) \
    ((This)->lpVtbl->SetRequestHeader(This, headerName, headerValue))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_get_ServerCredential(This, credential) \
    ((This)->lpVtbl->get_ServerCredential(This, credential))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_put_ServerCredential(This, credential) \
    ((This)->lpVtbl->put_ServerCredential(This, credential))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_get_ProxyCredential(This, credential) \
    ((This)->lpVtbl->get_ProxyCredential(This, credential))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_put_ProxyCredential(This, credential) \
    ((This)->lpVtbl->put_ProxyCredential(This, credential))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_get_Method(This, value) \
    ((This)->lpVtbl->get_Method(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_put_Method(This, value) \
    ((This)->lpVtbl->put_Method(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_get_Group(This, value) \
    ((This)->lpVtbl->get_Group(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_put_Group(This, value) \
    ((This)->lpVtbl->put_Group(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_get_CostPolicy(This, value) \
    ((This)->lpVtbl->get_CostPolicy(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_put_CostPolicy(This, value) \
    ((This)->lpVtbl->put_CostPolicy(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferBase_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferCompletionGroup[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Trigger)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This,
        __x_ABI_CWindows_CApplicationModel_CBackground_CIBackgroundTrigger** value);
    HRESULT (STDMETHODCALLTYPE* get_IsEnabled)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* Enable)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_get_Trigger(This, value) \
    ((This)->lpVtbl->get_Trigger(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_get_IsEnabled(This, value) \
    ((This)->lpVtbl->get_IsEnabled(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_Enable(This) \
    ((This)->lpVtbl->Enable(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferCompletionGroupTriggerDetails[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Downloads)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This,
        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** value);
    HRESULT (STDMETHODCALLTYPE* get_Uploads)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails* This,
        __FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetailsVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_get_Downloads(This, value) \
    ((This)->lpVtbl->get_Downloads(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_get_Uploads(This, value) \
    ((This)->lpVtbl->get_Uploads(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroupTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferContentPart[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetHeader)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        HSTRING headerName,
        HSTRING headerValue);
    HRESULT (STDMETHODCALLTYPE* SetText)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* SetFile)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_SetHeader(This, headerName, headerValue) \
    ((This)->lpVtbl->SetHeader(This, headerName, headerValue))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_SetText(This, value) \
    ((This)->lpVtbl->SetText(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_SetFile(This, value) \
    ((This)->lpVtbl->SetFile(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferContentPartFactory[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This,
        HSTRING name,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart** value);
    HRESULT (STDMETHODCALLTYPE* CreateWithNameAndFileName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory* This,
        HSTRING name,
        HSTRING fileName,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPart** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_CreateWithName(This, name, value) \
    ((This)->lpVtbl->CreateWithName(This, name, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_CreateWithNameAndFileName(This, name, fileName, value) \
    ((This)->lpVtbl->CreateWithNameAndFileName(This, name, fileName, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferContentPartFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferError
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferErrorStaticMethods[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethodsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetStatus)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods* This,
        INT32 hresult,
        enum __x_ABI_CWindows_CWeb_CWebErrorStatus* status);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethodsVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethodsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_GetStatus(This, hresult, status) \
    ((This)->lpVtbl->GetStatus(This, hresult, status))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferErrorStaticMethods_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferGroup[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TransferBehavior)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferBehavior* value);
    HRESULT (STDMETHODCALLTYPE* put_TransferBehavior)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferBehavior value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_get_TransferBehavior(This, value) \
    ((This)->lpVtbl->get_TransferBehavior(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_put_TransferBehavior(This, value) \
    ((This)->lpVtbl->put_TransferBehavior(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferGroupStatics[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics* This,
        HSTRING name,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStaticsVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_CreateGroup(This, name, value) \
    ((This)->lpVtbl->CreateGroup(This, name, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroupStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferOperation[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Guid)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_RequestedUri)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Method)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* get_Group)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CostPolicy)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy* value);
    HRESULT (STDMETHODCALLTYPE* put_CostPolicy)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferCostPolicy value);
    HRESULT (STDMETHODCALLTYPE* GetResultStreamAt)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        UINT64 position,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream** value);
    HRESULT (STDMETHODCALLTYPE* GetResponseInformation)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_get_Guid(This, value) \
    ((This)->lpVtbl->get_Guid(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_get_RequestedUri(This, value) \
    ((This)->lpVtbl->get_RequestedUri(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_get_Method(This, value) \
    ((This)->lpVtbl->get_Method(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Group may be altered or unavailable for releases after Windows 8.1. Instead, use TransferGroup.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_get_Group(This, value) \
    ((This)->lpVtbl->get_Group(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_get_CostPolicy(This, value) \
    ((This)->lpVtbl->get_CostPolicy(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_put_CostPolicy(This, value) \
    ((This)->lpVtbl->put_CostPolicy(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_GetResultStreamAt(This, position, value) \
    ((This)->lpVtbl->GetResultStreamAt(This, position, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_GetResponseInformation(This, value) \
    ((This)->lpVtbl->GetResponseInformation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferOperationPriority[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriorityVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Priority)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferPriority* value);
    HRESULT (STDMETHODCALLTYPE* put_Priority)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority* This,
        enum __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundTransferPriority value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriorityVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriorityVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_get_Priority(This, value) \
    ((This)->lpVtbl->get_Priority(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_put_Priority(This, value) \
    ((This)->lpVtbl->put_Priority(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferOperationPriority_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundTransferRangesDownloadedEventArgs[] = L"Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_WasDownloadRestarted)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AddedRanges)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgsVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_get_WasDownloadRestarted(This, value) \
    ((This)->lpVtbl->get_WasDownloadRestarted(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_get_AddedRanges(This, value) \
    ((This)->lpVtbl->get_AddedRanges(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferRangesDownloadedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploader[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploader";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateUpload)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CIStorageFile* sourceFile,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* CreateUploadFromStreamAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __x_ABI_CWindows_CStorage_CStreams_CIInputStream* sourceStream,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* CreateUploadWithFormDataAndAutoBoundaryAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* parts,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* CreateUploadWithSubTypeAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* parts,
        HSTRING subType,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* CreateUploadWithSubTypeAndBoundaryAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferContentPart* parts,
        HSTRING subType,
        HSTRING boundary,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_CreateUpload(This, uri, sourceFile, operation) \
    ((This)->lpVtbl->CreateUpload(This, uri, sourceFile, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_CreateUploadFromStreamAsync(This, uri, sourceStream, operation) \
    ((This)->lpVtbl->CreateUploadFromStreamAsync(This, uri, sourceStream, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_CreateUploadWithFormDataAndAutoBoundaryAsync(This, uri, parts, operation) \
    ((This)->lpVtbl->CreateUploadWithFormDataAndAutoBoundaryAsync(This, uri, parts, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_CreateUploadWithSubTypeAsync(This, uri, parts, subType, operation) \
    ((This)->lpVtbl->CreateUploadWithSubTypeAsync(This, uri, parts, subType, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_CreateUploadWithSubTypeAndBoundaryAsync(This, uri, parts, subType, boundary, operation) \
    ((This)->lpVtbl->CreateUploadWithSubTypeAndBoundaryAsync(This, uri, parts, subType, boundary, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploader2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploader2";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup** value);
    HRESULT (STDMETHODCALLTYPE* put_TransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* value);
    HRESULT (STDMETHODCALLTYPE* get_SuccessToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_SuccessToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* value);
    HRESULT (STDMETHODCALLTYPE* get_FailureToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_FailureToastNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CIToastNotification* value);
    HRESULT (STDMETHODCALLTYPE* get_SuccessTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_SuccessTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification* value);
    HRESULT (STDMETHODCALLTYPE* get_FailureTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification** value);
    HRESULT (STDMETHODCALLTYPE* put_FailureTileNotification)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2* This,
        __x_ABI_CWindows_CUI_CNotifications_CITileNotification* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_get_TransferGroup(This, value) \
    ((This)->lpVtbl->get_TransferGroup(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_put_TransferGroup(This, value) \
    ((This)->lpVtbl->put_TransferGroup(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_get_SuccessToastNotification(This, value) \
    ((This)->lpVtbl->get_SuccessToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_put_SuccessToastNotification(This, value) \
    ((This)->lpVtbl->put_SuccessToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_get_FailureToastNotification(This, value) \
    ((This)->lpVtbl->get_FailureToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_put_FailureToastNotification(This, value) \
    ((This)->lpVtbl->put_FailureToastNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_get_SuccessTileNotification(This, value) \
    ((This)->lpVtbl->get_SuccessTileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_put_SuccessTileNotification(This, value) \
    ((This)->lpVtbl->put_SuccessTileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_get_FailureTileNotification(This, value) \
    ((This)->lpVtbl->get_FailureTileNotification(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_put_FailureTileNotification(This, value) \
    ((This)->lpVtbl->put_FailureTileNotification(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploader3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploader3[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploader3";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CompletionGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_get_CompletionGroup(This, value) \
    ((This)->lpVtbl->get_CompletionGroup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderFactory[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWithCompletionGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferCompletionGroup* completionGroup,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploader** backgroundUploader);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactoryVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_CreateWithCompletionGroup(This, completionGroup, backgroundUploader) \
    ((This)->lpVtbl->CreateWithCompletionGroup(This, completionGroup, backgroundUploader))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderStaticMethods[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethodsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentUploadsAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetCurrentUploadsAsync(group) may be altered or unavailable for releases after Windows 8.1. Instead, use GetCurrentUploadsForTransferGroupAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* GetCurrentUploadsForGroupAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods* This,
        HSTRING group,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethodsVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethodsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_GetCurrentUploadsAsync(This, operation) \
    ((This)->lpVtbl->GetCurrentUploadsAsync(This, operation))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("GetCurrentUploadsAsync(group) may be altered or unavailable for releases after Windows 8.1. Instead, use GetCurrentUploadsForTransferGroupAsync.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_GetCurrentUploadsForGroupAsync(This, group, operation) \
    ((This)->lpVtbl->GetCurrentUploadsForGroupAsync(This, group, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderStaticMethods2[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetCurrentUploadsForTransferGroupAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup* group,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_GetCurrentUploadsForTransferGroupAsync(This, group, operation) \
    ((This)->lpVtbl->GetCurrentUploadsForTransferGroupAsync(This, group, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderStaticMethods2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IBackgroundUploaderUserConsent[] = L"Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("IBackgroundUploaderUserConsent is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("RequestUnconstrainedUploadsAsync is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* RequestUnconstrainedUploadsAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent* This,
        __FIIterable_1_Windows__CNetworking__CBackgroundTransfer__CUploadOperation* operations,
        __FIAsyncOperation_1_Windows__CNetworking__CBackgroundTransfer__CUnconstrainedTransferRequestResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsentVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("RequestUnconstrainedUploadsAsync is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_RequestUnconstrainedUploadsAsync(This, operations, operation) \
    ((This)->lpVtbl->RequestUnconstrainedUploadsAsync(This, operations, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundUploaderUserConsent_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IContentPrefetcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.ContentPrefetcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IContentPrefetcher[] = L"Windows.Networking.BackgroundTransfer.IContentPrefetcher";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentUris)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        __FIVector_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* put_IndirectContentUri)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_IndirectContentUri)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_get_ContentUris(This, value) \
    ((This)->lpVtbl->get_ContentUris(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_put_IndirectContentUri(This, value) \
    ((This)->lpVtbl->put_IndirectContentUri(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_get_IndirectContentUri(This, value) \
    ((This)->lpVtbl->get_IndirectContentUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcher_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IContentPrefetcherTime
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.ContentPrefetcher
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IContentPrefetcherTime[] = L"Windows.Networking.BackgroundTransfer.IContentPrefetcherTime";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTimeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LastSuccessfulPrefetchTime)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTimeVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTimeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_get_LastSuccessfulPrefetchTime(This, value) \
    ((This)->lpVtbl->get_LastSuccessfulPrefetchTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIContentPrefetcherTime_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ResultFile)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundDownloadProgress* value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* AttachAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This);
    HRESULT (STDMETHODCALLTYPE* Resume)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperationVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_get_ResultFile(This, value) \
    ((This)->lpVtbl->get_ResultFile(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_AttachAsync(This, operation) \
    ((This)->lpVtbl->AttachAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_Resume(This) \
    ((This)->lpVtbl->Resume(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation2[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation2";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_get_TransferGroup(This, value) \
    ((This)->lpVtbl->get_TransferGroup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation3[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation3";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsRandomAccessRequired)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsRandomAccessRequired)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetResultRandomAccessStreamReference)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** stream);
    HRESULT (STDMETHODCALLTYPE* GetDownloadedRanges)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        __FIVector_1_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferFileRange** value);
    HRESULT (STDMETHODCALLTYPE* add_RangesDownloaded)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        __FITypedEventHandler_2_Windows__CNetworking__CBackgroundTransfer__CDownloadOperation_Windows__CNetworking__CBackgroundTransfer__CBackgroundTransferRangesDownloadedEventArgs* eventHandler,
        EventRegistrationToken* eventCookie);
    HRESULT (STDMETHODCALLTYPE* remove_RangesDownloaded)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        EventRegistrationToken eventCookie);
    HRESULT (STDMETHODCALLTYPE* put_RequestedUri)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_RecoverableWebErrorStatuses)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        __FIVector_1_Windows__CWeb__CWebErrorStatus** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentWebErrorStatus)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3* This,
        __FIReference_1_Windows__CWeb__CWebErrorStatus** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_get_IsRandomAccessRequired(This, value) \
    ((This)->lpVtbl->get_IsRandomAccessRequired(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_put_IsRandomAccessRequired(This, value) \
    ((This)->lpVtbl->put_IsRandomAccessRequired(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_GetResultRandomAccessStreamReference(This, stream) \
    ((This)->lpVtbl->GetResultRandomAccessStreamReference(This, stream))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_GetDownloadedRanges(This, value) \
    ((This)->lpVtbl->GetDownloadedRanges(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_add_RangesDownloaded(This, eventHandler, eventCookie) \
    ((This)->lpVtbl->add_RangesDownloaded(This, eventHandler, eventCookie))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_remove_RangesDownloaded(This, eventCookie) \
    ((This)->lpVtbl->remove_RangesDownloaded(This, eventCookie))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_put_RequestedUri(This, value) \
    ((This)->lpVtbl->put_RequestedUri(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_get_RecoverableWebErrorStatuses(This, value) \
    ((This)->lpVtbl->get_RecoverableWebErrorStatuses(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_get_CurrentWebErrorStatus(This, value) \
    ((This)->lpVtbl->get_CurrentWebErrorStatus(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation4[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation4";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* MakeCurrentInTransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_MakeCurrentInTransferGroup(This) \
    ((This)->lpVtbl->MakeCurrentInTransferGroup(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IDownloadOperation5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IDownloadOperation5[] = L"Windows.Networking.BackgroundTransfer.IDownloadOperation5";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetRequestHeader)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This,
        HSTRING headerName,
        HSTRING headerValue);
    HRESULT (STDMETHODCALLTYPE* RemoveRequestHeader)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5* This,
        HSTRING headerName);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_SetRequestHeader(This, headerName, headerValue) \
    ((This)->lpVtbl->SetRequestHeader(This, headerName, headerValue))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_RemoveRequestHeader(This, headerName) \
    ((This)->lpVtbl->RemoveRequestHeader(This, headerName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIDownloadOperation5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IResponseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.ResponseInformation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IResponseInformation[] = L"Windows.Networking.BackgroundTransfer.IResponseInformation";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsResumable)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ActualUri)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_StatusCode)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Headers)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation* This,
        __FIMapView_2_HSTRING_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformationVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_get_IsResumable(This, value) \
    ((This)->lpVtbl->get_IsResumable(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_get_ActualUri(This, value) \
    ((This)->lpVtbl->get_ActualUri(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_get_StatusCode(This, value) \
    ((This)->lpVtbl->get_StatusCode(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_get_Headers(This, value) \
    ((This)->lpVtbl->get_Headers(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIResponseInformation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUnconstrainedTransferRequestResult[] = L"Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("IUnconstrainedTransferRequestResult is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("IsUnconstrained is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_IsUnconstrained)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResultVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("IsUnconstrained is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_get_IsUnconstrained(This, value) \
    ((This)->lpVtbl->get_IsUnconstrained(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUnconstrainedTransferRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SourceFile)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        __x_ABI_CWindows_CStorage_CIStorageFile** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CBackgroundUploadProgress* value);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);
    HRESULT (STDMETHODCALLTYPE* AttachAsync)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation* This,
        __FIAsyncOperationWithProgress_2_Windows__CNetworking__CBackgroundTransfer__CUploadOperation_Windows__CNetworking__CBackgroundTransfer__CUploadOperation** operation);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperationVtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_get_SourceFile(This, value) \
    ((This)->lpVtbl->get_SourceFile(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_AttachAsync(This, operation) \
    ((This)->lpVtbl->AttachAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation2[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation2";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_TransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2* This,
        __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIBackgroundTransferGroup** value);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_get_TransferGroup(This, value) \
    ((This)->lpVtbl->get_TransferGroup(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation3[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation3";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* MakeCurrentInTransferGroup)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3* This);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_MakeCurrentInTransferGroup(This) \
    ((This)->lpVtbl->MakeCurrentInTransferGroup(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Networking.BackgroundTransfer.IUploadOperation4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Networking.BackgroundTransfer.UploadOperation
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Networking_BackgroundTransfer_IUploadOperation4[] = L"Windows.Networking.BackgroundTransfer.IUploadOperation4";
typedef struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetRequestHeader)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This,
        HSTRING headerName,
        HSTRING headerValue);
    HRESULT (STDMETHODCALLTYPE* RemoveRequestHeader)(__x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4* This,
        HSTRING headerName);

    END_INTERFACE
} __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4Vtbl;

interface __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4
{
    CONST_VTBL struct __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_SetRequestHeader(This, headerName, headerValue) \
    ((This)->lpVtbl->SetRequestHeader(This, headerName, headerValue))

#define __x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_RemoveRequestHeader(This, headerName) \
    ((This)->lpVtbl->RemoveRequestHeader(This, headerName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4;
#endif /* !defined(____x_ABI_CWindows_CNetworking_CBackgroundTransfer_CIUploadOperation4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundDownloader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderUserConsent interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundDownloaderStaticMethods interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundDownloader ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *    Windows.Networking.BackgroundTransfer.IBackgroundDownloader2
 *    Windows.Networking.BackgroundTransfer.IBackgroundDownloader3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundDownloader_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundDownloader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundDownloader[] = L"Windows.Networking.BackgroundTransfer.BackgroundDownloader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroup_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroup[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferCompletionGroupTriggerDetails ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroupTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroupTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferCompletionGroupTriggerDetails[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferCompletionGroupTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPartFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferContentPart ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferContentPart_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferContentPart_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferContentPart[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferContentPart";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferError
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundTransferErrorStaticMethods interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferError_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferError_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferError[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferError";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundTransferGroupStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferGroup_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferGroup[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferRangesDownloadedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferRangesDownloadedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundTransferRangesDownloadedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundTransferRangesDownloadedEventArgs[] = L"Windows.Networking.BackgroundTransfer.BackgroundTransferRangesDownloadedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.BackgroundUploader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Networking.BackgroundTransfer.IBackgroundUploaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundUploaderUserConsent interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IBackgroundUploaderStaticMethods interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IBackgroundUploader ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferBase
 *    Windows.Networking.BackgroundTransfer.IBackgroundUploader2
 *    Windows.Networking.BackgroundTransfer.IBackgroundUploader3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundUploader_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_BackgroundUploader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_BackgroundUploader[] = L"Windows.Networking.BackgroundTransfer.BackgroundUploader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.ContentPrefetcher
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IContentPrefetcher interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Networking.BackgroundTransfer.IContentPrefetcherTime interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ContentPrefetcher_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ContentPrefetcher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_ContentPrefetcher[] = L"Windows.Networking.BackgroundTransfer.ContentPrefetcher";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.DownloadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation2
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation3
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation4
 *    Windows.Networking.BackgroundTransfer.IDownloadOperation5
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_DownloadOperation_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_DownloadOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_DownloadOperation[] = L"Windows.Networking.BackgroundTransfer.DownloadOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.ResponseInformation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IResponseInformation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ResponseInformation_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_ResponseInformation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_ResponseInformation[] = L"Windows.Networking.BackgroundTransfer.ResponseInformation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IUnconstrainedTransferRequestResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UnconstrainedTransferRequestResult_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UnconstrainedTransferRequestResult_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("UnconstrainedTransferRequestResult is deprecated and may not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_UnconstrainedTransferRequestResult[] = L"Windows.Networking.BackgroundTransfer.UnconstrainedTransferRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Networking.BackgroundTransfer.UploadOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Networking.BackgroundTransfer.IUploadOperation ** Default Interface **
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperation
 *    Windows.Networking.BackgroundTransfer.IBackgroundTransferOperationPriority
 *    Windows.Networking.BackgroundTransfer.IUploadOperation2
 *    Windows.Networking.BackgroundTransfer.IUploadOperation3
 *    Windows.Networking.BackgroundTransfer.IUploadOperation4
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UploadOperation_DEFINED
#define RUNTIMECLASS_Windows_Networking_BackgroundTransfer_UploadOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Networking_BackgroundTransfer_UploadOperation[] = L"Windows.Networking.BackgroundTransfer.UploadOperation";
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
#endif // __windows2Enetworking2Ebackgroundtransfer_p_h__

#endif // __windows2Enetworking2Ebackgroundtransfer_h__
