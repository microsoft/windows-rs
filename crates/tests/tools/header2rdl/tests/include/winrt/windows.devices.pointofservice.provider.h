
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
#ifndef __windows2Edevices2Epointofservice2Eprovider_h__
#define __windows2Edevices2Epointofservice2Eprovider_h__
#ifndef __windows2Edevices2Epointofservice2Eprovider_p_h__
#define __windows2Edevices2Epointofservice2Eprovider_p_h__


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
#include "Windows.Devices.PointOfService.h"
#include "Windows.Graphics.Imaging.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerDisableScannerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerDisableScannerRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerDisableScannerRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerDisableScannerRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerDisableScannerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerDisableScannerRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerEnableScannerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerEnableScannerRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerEnableScannerRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerEnableScannerRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerEnableScannerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerEnableScannerRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerFrameReader;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerFrameReader

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerFrameReaderFrameArrivedEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerFrameReaderFrameArrivedEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerGetSymbologyAttributesRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerGetSymbologyAttributesRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerGetSymbologyAttributesRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerGetSymbologyAttributesRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerGetSymbologyAttributesRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerGetSymbologyAttributesRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerHideVideoPreviewRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerHideVideoPreviewRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerHideVideoPreviewRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerHideVideoPreviewRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerHideVideoPreviewRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerHideVideoPreviewRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerProviderConnection;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerProviderConnection2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerProviderTriggerDetails;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderTriggerDetails

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerSetActiveSymbologiesRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetActiveSymbologiesRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerSetActiveSymbologiesRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetActiveSymbologiesRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerSetActiveSymbologiesRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetActiveSymbologiesRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerSetSymbologyAttributesRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetSymbologyAttributesRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerSetSymbologyAttributesRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetSymbologyAttributesRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerSetSymbologyAttributesRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetSymbologyAttributesRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerStartSoftwareTriggerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStartSoftwareTriggerRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerStartSoftwareTriggerRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStartSoftwareTriggerRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerStartSoftwareTriggerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStartSoftwareTriggerRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerStopSoftwareTriggerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStopSoftwareTriggerRequest

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerStopSoftwareTriggerRequest2;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2 ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStopSoftwareTriggerRequest2

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerStopSoftwareTriggerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStopSoftwareTriggerRequestEventArgs

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeScannerVideoFrame;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerVideoFrame

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    interface IBarcodeSymbologyAttributesBuilder;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder ABI::Windows::Devices::PointOfService::Provider::IBarcodeSymbologyAttributesBuilder

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_FWD_DEFINED__

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


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerFrameReader;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bfdc3725-f790-54a2-8305-8994b0979e0a"))
IAsyncOperation<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerFrameReader*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*> __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_t;
#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("282ade1e-dffd-5e70-a29e-7a514fc2d779"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerFrameReader*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerVideoFrame;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_USE
#define DEF___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("33e8319b-2bd5-502e-96f4-6465456ca13a"))
IAsyncOperation<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerVideoFrame*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerVideoFrame*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerVideoFrame*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerVideoFrame*> __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_t;
#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2004d87f-6e77-5658-9092-92a3f1cce4c4"))
IAsyncOperationCompletedHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerVideoFrame*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerVideoFrame*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerVideoFrame*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerVideoFrame*> __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000


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



#ifndef DEF___FIVector_1_UINT32_USE
#define DEF___FIVector_1_UINT32_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("534832ed-2a03-5604-890d-5a928cd427b9"))
IVector<UINT32> : IVector_impl<UINT32>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<UInt32>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<UINT32> __FIVector_1_UINT32_t;
#define __FIVector_1_UINT32 ABI::Windows::Foundation::Collections::__FIVector_1_UINT32_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_UINT32_USE */


namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerFrameReaderFrameArrivedEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("730d5c4a-54e7-57dd-aaa2-08527518c449"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReaderFrameArrivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerFrameReader*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReaderFrameArrivedEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerFrameReaderFrameArrivedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader, Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReader*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerFrameReaderFrameArrivedEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerProviderConnection;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerDisableScannerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d801fc9f-851f-5a8a-9558-a3df780339b0"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerDisableScannerRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerDisableScannerRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerDisableScannerRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerDisableScannerRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerEnableScannerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("44f1a2dc-3f2a-5338-b58e-fcc2709d07a7"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerEnableScannerRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerEnableScannerRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerEnableScannerRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerEnableScannerRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerGetSymbologyAttributesRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e590f9b4-1481-5945-b5bd-0ab4b10a4b5f"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerGetSymbologyAttributesRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerGetSymbologyAttributesRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerGetSymbologyAttributesRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerGetSymbologyAttributesRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerHideVideoPreviewRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f3679d04-d6d9-5e95-b674-1135bfd4f42f"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerHideVideoPreviewRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerHideVideoPreviewRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerHideVideoPreviewRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerHideVideoPreviewRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerSetActiveSymbologiesRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9dc3de50-d6f6-58b2-9093-d6090b8323fe"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerSetActiveSymbologiesRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerSetActiveSymbologiesRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetActiveSymbologiesRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerSetActiveSymbologiesRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerSetSymbologyAttributesRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0c9b13f0-5d9f-56ca-8989-0746bd811a21"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerSetSymbologyAttributesRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerSetSymbologyAttributesRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetSymbologyAttributesRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerSetSymbologyAttributesRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerStartSoftwareTriggerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("16220e79-8b03-5498-9983-b93568921676"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerStartSoftwareTriggerRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerStartSoftwareTriggerRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStartSoftwareTriggerRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerStartSoftwareTriggerRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerStopSoftwareTriggerRequestEventArgs;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d695be73-bf91-5036-95d2-159032c94cb3"))
ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerStopSoftwareTriggerRequestEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerStopSoftwareTriggerRequestEventArgs*, ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStopSoftwareTriggerRequestEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection, Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerProviderConnection*, ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerStopSoftwareTriggerRequestEventArgs*> __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_t;
#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                class BarcodeScannerReport;
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                interface IBarcodeScannerReport;
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport ABI::Windows::Devices::PointOfService::IBarcodeScannerReport

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                class BarcodeSymbologyAttributes;
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                interface IBarcodeSymbologyAttributes;
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes ABI::Windows::Devices::PointOfService::IBarcodeSymbologyAttributes

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                class UnifiedPosErrorData;
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                interface IUnifiedPosErrorData;
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData ABI::Windows::Devices::PointOfService::IUnifiedPosErrorData

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData_FWD_DEFINED__

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
                typedef struct BitmapSize BitmapSize;
            } /* Imaging */
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
            namespace PointOfService {
                namespace Provider {
                    typedef enum BarcodeScannerTriggerState : int BarcodeScannerTriggerState;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerDisableScannerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerEnableScannerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerGetSymbologyAttributesRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerHideVideoPreviewRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerSetActiveSymbologiesRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerSetSymbologyAttributesRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerStartSoftwareTriggerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    class BarcodeScannerStopSoftwareTriggerRequest;
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    enum BarcodeScannerTriggerState : int
                    {
                        BarcodeScannerTriggerState_Released = 0,
                        BarcodeScannerTriggerState_Pressed = 1,
                    };
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerDisableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("88ecf7c0-37b9-4275-8e77-c8e52ae5a9c8")
                    IBarcodeScannerDisableScannerRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerDisableScannerRequest = __uuidof(IBarcodeScannerDisableScannerRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerDisableScannerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("ccdfe625-65c3-4ccc-b457-f39c7a9ea60d")
                    IBarcodeScannerDisableScannerRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerDisableScannerRequest2 = __uuidof(IBarcodeScannerDisableScannerRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerDisableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("7006e142-e802-46f5-b604-352a15ce9232")
                    IBarcodeScannerDisableScannerRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerDisableScannerRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerDisableScannerRequestEventArgs = __uuidof(IBarcodeScannerDisableScannerRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerEnableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("c0b3e9ba-816a-452b-bd77-b7e453ec446d")
                    IBarcodeScannerEnableScannerRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerEnableScannerRequest = __uuidof(IBarcodeScannerEnableScannerRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerEnableScannerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("71a4f2a8-9905-41ac-9121-b645916a84a1")
                    IBarcodeScannerEnableScannerRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerEnableScannerRequest2 = __uuidof(IBarcodeScannerEnableScannerRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerEnableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("956c9419-7b4e-4451-8c41-8e10cfbc5b41")
                    IBarcodeScannerEnableScannerRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerEnableScannerRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerEnableScannerRequestEventArgs = __uuidof(IBarcodeScannerEnableScannerRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerFrameReader[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("dbc72b07-64c3-482b-93c8-65fb33c22208")
                    IBarcodeScannerFrameReader : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE StartAsync(
                            __FIAsyncOperation_1_boolean** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE StopAsync(
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE TryAcquireLatestFrameAsync(
                            __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_FrameArrived(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_FrameArrived(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerFrameReader = __uuidof(IBarcodeScannerFrameReader);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerFrameReaderFrameArrivedEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("b0bbd604-54fd-436d-8629-712e787223dd")
                    IBarcodeScannerFrameReaderFrameArrivedEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerFrameReaderFrameArrivedEventArgs = __uuidof(IBarcodeScannerFrameReaderFrameArrivedEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerGetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("9774c46a-58e4-4c5f-b8e9-e41467632700")
                    IBarcodeScannerGetSymbologyAttributesRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Symbology(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Devices::PointOfService::IBarcodeSymbologyAttributes* attributes,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerGetSymbologyAttributesRequest = __uuidof(IBarcodeScannerGetSymbologyAttributesRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerGetSymbologyAttributesRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("6a6a2b13-75a8-49fb-b852-bfb93d760af7")
                    IBarcodeScannerGetSymbologyAttributesRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerGetSymbologyAttributesRequest2 = __uuidof(IBarcodeScannerGetSymbologyAttributesRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerGetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("7f89de3e-fb5d-493c-b402-356b24d574a6")
                    IBarcodeScannerGetSymbologyAttributesRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerGetSymbologyAttributesRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerGetSymbologyAttributesRequestEventArgs = __uuidof(IBarcodeScannerGetSymbologyAttributesRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerHideVideoPreviewRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("fa4ebe7f-6670-40e1-b90b-bb10d8d425fa")
                    IBarcodeScannerHideVideoPreviewRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerHideVideoPreviewRequest = __uuidof(IBarcodeScannerHideVideoPreviewRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerHideVideoPreviewRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("7e28435d-9814-431d-a2f2-d6248c5ad4b5")
                    IBarcodeScannerHideVideoPreviewRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerHideVideoPreviewRequest2 = __uuidof(IBarcodeScannerHideVideoPreviewRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerHideVideoPreviewRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("16a281fc-d6be-4bc7-9df1-33741f3eadea")
                    IBarcodeScannerHideVideoPreviewRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerHideVideoPreviewRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerHideVideoPreviewRequestEventArgs = __uuidof(IBarcodeScannerHideVideoPreviewRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerProviderConnection[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("b44acbed-0b3a-4fa3-86c5-491ea30780eb")
                    IBarcodeScannerProviderConnection : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Id(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_VideoDeviceId(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedSymbologies(
                            __FIVector_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CompanyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_CompanyName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Name(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Name(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Version(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Version(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE Start(void) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportScannedDataAsync(
                            ABI::Windows::Devices::PointOfService::IBarcodeScannerReport* report,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportTriggerStateAsync(
                            ABI::Windows::Devices::PointOfService::Provider::BarcodeScannerTriggerState state,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportErrorAsync(
                            ABI::Windows::Devices::PointOfService::IUnifiedPosErrorData* errorData,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportErrorAsyncWithScanReport(
                            ABI::Windows::Devices::PointOfService::IUnifiedPosErrorData* errorData,
                            boolean isRetriable,
                            ABI::Windows::Devices::PointOfService::IBarcodeScannerReport* scanReport,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_EnableScannerRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_EnableScannerRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_DisableScannerRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_DisableScannerRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SetActiveSymbologiesRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SetActiveSymbologiesRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StartSoftwareTriggerRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StartSoftwareTriggerRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_StopSoftwareTriggerRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_StopSoftwareTriggerRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_GetBarcodeSymbologyAttributesRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_GetBarcodeSymbologyAttributesRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_SetBarcodeSymbologyAttributesRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_SetBarcodeSymbologyAttributesRequested(
                            EventRegistrationToken token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE add_HideVideoPreviewRequested(
                            __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE remove_HideVideoPreviewRequested(
                            EventRegistrationToken token
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerProviderConnection = __uuidof(IBarcodeScannerProviderConnection);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerProviderConnection2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("be9b53cd-1134-418c-a06b-04423a73f3d7")
                    IBarcodeScannerProviderConnection2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateFrameReaderAsync(
                            __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFrameReaderWithFormatAsync(
                            ABI::Windows::Graphics::Imaging::BitmapPixelFormat preferredFormat,
                            __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateFrameReaderWithFormatAndSizeAsync(
                            ABI::Windows::Graphics::Imaging::BitmapPixelFormat preferredFormat,
                            ABI::Windows::Graphics::Imaging::BitmapSize preferredSize,
                            __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerProviderConnection2 = __uuidof(IBarcodeScannerProviderConnection2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerProviderTriggerDetails[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("50856d82-24e3-48ce-99c7-70aac1cbc9f7")
                    IBarcodeScannerProviderTriggerDetails : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Connection(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerProviderConnection** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerProviderTriggerDetails = __uuidof(IBarcodeScannerProviderTriggerDetails);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetActiveSymbologiesRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("db3f32b9-f7da-41a1-9f79-07bcd95f0bdf")
                    IBarcodeScannerSetActiveSymbologiesRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Symbologies(
                            __FIVectorView_1_UINT32** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerSetActiveSymbologiesRequest = __uuidof(IBarcodeScannerSetActiveSymbologiesRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetActiveSymbologiesRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("f5ff6edf-fa9a-4749-b11b-e8fccb75bc6b")
                    IBarcodeScannerSetActiveSymbologiesRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerSetActiveSymbologiesRequest2 = __uuidof(IBarcodeScannerSetActiveSymbologiesRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetActiveSymbologiesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("06305afa-7bf6-4d52-801a-330272f60ae1")
                    IBarcodeScannerSetActiveSymbologiesRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetActiveSymbologiesRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerSetActiveSymbologiesRequestEventArgs = __uuidof(IBarcodeScannerSetActiveSymbologiesRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("32fb814f-a37f-48b0-acea-dce1480f12ae")
                    IBarcodeScannerSetSymbologyAttributesRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Symbology(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Attributes(
                            ABI::Windows::Devices::PointOfService::IBarcodeSymbologyAttributes** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerSetSymbologyAttributesRequest = __uuidof(IBarcodeScannerSetSymbologyAttributesRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetSymbologyAttributesRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("dffbbfc1-dba8-4b77-be1e-b56cd72f65b3")
                    IBarcodeScannerSetSymbologyAttributesRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerSetSymbologyAttributesRequest2 = __uuidof(IBarcodeScannerSetSymbologyAttributesRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("b2b89809-9824-47d4-85bd-d0077baa7bd2")
                    IBarcodeScannerSetSymbologyAttributesRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerSetSymbologyAttributesRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerSetSymbologyAttributesRequestEventArgs = __uuidof(IBarcodeScannerSetSymbologyAttributesRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStartSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("e3fa7b27-ff62-4454-af4a-cb6144a3e3f7")
                    IBarcodeScannerStartSoftwareTriggerRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerStartSoftwareTriggerRequest = __uuidof(IBarcodeScannerStartSoftwareTriggerRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStartSoftwareTriggerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("eb72a25c-6658-4765-a68e-327482653deb")
                    IBarcodeScannerStartSoftwareTriggerRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerStartSoftwareTriggerRequest2 = __uuidof(IBarcodeScannerStartSoftwareTriggerRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStartSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("2305d843-c88f-4f3b-8c3b-d3df071051ec")
                    IBarcodeScannerStartSoftwareTriggerRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStartSoftwareTriggerRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerStartSoftwareTriggerRequestEventArgs = __uuidof(IBarcodeScannerStartSoftwareTriggerRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStopSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("6f9faf35-e287-4ca8-b70d-5a91d694f668")
                    IBarcodeScannerStopSoftwareTriggerRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportCompletedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedAsync(
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerStopSoftwareTriggerRequest = __uuidof(IBarcodeScannerStopSoftwareTriggerRequest);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStopSoftwareTriggerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("cb57c5dd-fe50-49f8-a0b4-bdc230814da2")
                    IBarcodeScannerStopSoftwareTriggerRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAsync(
                            INT32 reason,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ReportFailedWithFailedReasonAndDescriptionAsync(
                            INT32 reason,
                            HSTRING failedReasonDescription,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerStopSoftwareTriggerRequest2 = __uuidof(IBarcodeScannerStopSoftwareTriggerRequest2);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStopSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("eac34450-4eb7-481a-9273-147a273b99b8")
                    IBarcodeScannerStopSoftwareTriggerRequestEventArgs : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Request(
                            ABI::Windows::Devices::PointOfService::Provider::IBarcodeScannerStopSoftwareTriggerRequest** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                            ABI::Windows::Foundation::IDeferral** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerStopSoftwareTriggerRequestEventArgs = __uuidof(IBarcodeScannerStopSoftwareTriggerRequestEventArgs);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerVideoFrame[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("7e585248-9df7-4121-a175-801d8000112e")
                    IBarcodeScannerVideoFrame : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Format(
                            ABI::Windows::Graphics::Imaging::BitmapPixelFormat* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Width(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Height(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PixelData(
                            ABI::Windows::Storage::Streams::IBuffer** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeScannerVideoFrame = __uuidof(IBarcodeScannerVideoFrame);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeSymbologyAttributesBuilder[] = L"Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder";
namespace ABI {
    namespace Windows {
        namespace Devices {
            namespace PointOfService {
                namespace Provider {
                    MIDL_INTERFACE("c57b0cbf-e4f5-40b9-84cf-e63fbaea42b4")
                    IBarcodeSymbologyAttributesBuilder : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_IsCheckDigitValidationSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsCheckDigitValidationSupported(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsCheckDigitTransmissionSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsCheckDigitTransmissionSupported(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IsDecodeLengthSupported(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_IsDecodeLengthSupported(
                            boolean value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateAttributes(
                            ABI::Windows::Devices::PointOfService::IBarcodeSymbologyAttributes** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IBarcodeSymbologyAttributesBuilder = __uuidof(IBarcodeSymbologyAttributesBuilder);
                } /* Provider */
            } /* PointOfService */
        } /* Devices */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReader_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReader[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReaderFrameArrivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReaderFrameArrivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReaderFrameArrivedEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderConnection[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderTriggerDetails[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerVideoFrame_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerVideoFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerVideoFrame[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeSymbologyAttributesBuilder_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeSymbologyAttributesBuilder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeSymbologyAttributesBuilder[] = L"Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2 __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* This,
        __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame;

typedef struct __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrameVtbl;

interface __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* This,
        __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrameVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

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

#if !defined(____FIVector_1_UINT32_INTERFACE_DEFINED__)
#define ____FIVector_1_UINT32_INTERFACE_DEFINED__

typedef interface __FIVector_1_UINT32 __FIVector_1_UINT32;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_UINT32;

typedef struct __FIVector_1_UINT32Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_UINT32* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_UINT32* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_UINT32* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_UINT32* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_UINT32* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_UINT32* This,
        UINT32 index,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_UINT32* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_UINT32* This,
        __FIVectorView_1_UINT32** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_UINT32* This,
        UINT32 value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_UINT32* This,
        UINT32 index,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_UINT32* This,
        UINT32 index,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_UINT32* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_UINT32* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_UINT32* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_UINT32* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        UINT32* items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_UINT32* This,
        UINT32 itemsLength,
        UINT32* items);

    END_INTERFACE
} __FIVector_1_UINT32Vtbl;

interface __FIVector_1_UINT32
{
    CONST_VTBL struct __FIVector_1_UINT32Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_UINT32_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_UINT32_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_UINT32_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_UINT32_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_UINT32_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_UINT32_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_UINT32_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_UINT32_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_UINT32_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_UINT32_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_UINT32_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_UINT32_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_UINT32_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_UINT32_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_UINT32_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_UINT32_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_UINT32_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_UINT32_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_UINT32_INTERFACE_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* sender,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData_FWD_DEFINED__
#define ____x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData __x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData;

#endif // ____x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData_FWD_DEFINED__

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

typedef enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat;

typedef struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize;

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIBuffer __x_ABI_CWindows_CStorage_CStreams_CIBuffer;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIBuffer_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CBarcodeScannerTriggerState __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CBarcodeScannerTriggerState;

/*
 *
 * Struct Windows.Devices.PointOfService.Provider.BarcodeScannerTriggerState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CBarcodeScannerTriggerState
{
    BarcodeScannerTriggerState_Released = 0,
    BarcodeScannerTriggerState_Pressed = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerDisableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerDisableScannerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerDisableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerDisableScannerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerEnableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerEnableScannerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerEnableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerEnableScannerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerFrameReader[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        __FIAsyncOperation_1_boolean** operation);
    HRESULT (STDMETHODCALLTYPE* StopAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* TryAcquireLatestFrameAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerVideoFrame** operation);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection** value);
    HRESULT (STDMETHODCALLTYPE* add_FrameArrived)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReaderFrameArrivedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_FrameArrived)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_StartAsync(This, operation) \
    ((This)->lpVtbl->StartAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_StopAsync(This, operation) \
    ((This)->lpVtbl->StopAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_TryAcquireLatestFrameAsync(This, operation) \
    ((This)->lpVtbl->TryAcquireLatestFrameAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_add_FrameArrived(This, handler, token) \
    ((This)->lpVtbl->add_FrameArrived(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_remove_FrameArrived(This, token) \
    ((This)->lpVtbl->remove_FrameArrived(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerFrameReaderFrameArrivedEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerFrameReaderFrameArrivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerGetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Symbology)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes* attributes,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_get_Symbology(This, value) \
    ((This)->lpVtbl->get_Symbology(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_ReportCompletedAsync(This, attributes, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, attributes, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerGetSymbologyAttributesRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerGetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerGetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerHideVideoPreviewRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerHideVideoPreviewRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerHideVideoPreviewRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerHideVideoPreviewRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerProviderConnection[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnectionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VideoDeviceId)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_SupportedSymbologies)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FIVector_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* get_CompanyName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_CompanyName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Name)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Version)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* Start)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This);
    HRESULT (STDMETHODCALLTYPE* ReportScannedDataAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport* report,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportTriggerStateAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        enum __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CBarcodeScannerTriggerState state,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportErrorAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData* errorData,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportErrorAsyncWithScanReport)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CIUnifiedPosErrorData* errorData,
        boolean isRetriable,
        __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeScannerReport* scanReport,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* add_EnableScannerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerEnableScannerRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_EnableScannerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_DisableScannerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerDisableScannerRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_DisableScannerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SetActiveSymbologiesRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetActiveSymbologiesRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SetActiveSymbologiesRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_StartSoftwareTriggerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStartSoftwareTriggerRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StartSoftwareTriggerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_StopSoftwareTriggerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerStopSoftwareTriggerRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_StopSoftwareTriggerRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_GetBarcodeSymbologyAttributesRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerGetSymbologyAttributesRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_GetBarcodeSymbologyAttributesRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_SetBarcodeSymbologyAttributesRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerSetSymbologyAttributesRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_SetBarcodeSymbologyAttributesRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_HideVideoPreviewRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        __FITypedEventHandler_2_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerProviderConnection_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerHideVideoPreviewRequestEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_HideVideoPreviewRequested)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnectionVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnectionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_get_VideoDeviceId(This, value) \
    ((This)->lpVtbl->get_VideoDeviceId(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_get_SupportedSymbologies(This, value) \
    ((This)->lpVtbl->get_SupportedSymbologies(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_get_CompanyName(This, value) \
    ((This)->lpVtbl->get_CompanyName(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_put_CompanyName(This, value) \
    ((This)->lpVtbl->put_CompanyName(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_put_Name(This, value) \
    ((This)->lpVtbl->put_Name(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_put_Version(This, value) \
    ((This)->lpVtbl->put_Version(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_Start(This) \
    ((This)->lpVtbl->Start(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_ReportScannedDataAsync(This, report, operation) \
    ((This)->lpVtbl->ReportScannedDataAsync(This, report, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_ReportTriggerStateAsync(This, state, operation) \
    ((This)->lpVtbl->ReportTriggerStateAsync(This, state, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_ReportErrorAsync(This, errorData, operation) \
    ((This)->lpVtbl->ReportErrorAsync(This, errorData, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_ReportErrorAsyncWithScanReport(This, errorData, isRetriable, scanReport, operation) \
    ((This)->lpVtbl->ReportErrorAsyncWithScanReport(This, errorData, isRetriable, scanReport, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_EnableScannerRequested(This, handler, token) \
    ((This)->lpVtbl->add_EnableScannerRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_EnableScannerRequested(This, token) \
    ((This)->lpVtbl->remove_EnableScannerRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_DisableScannerRequested(This, handler, token) \
    ((This)->lpVtbl->add_DisableScannerRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_DisableScannerRequested(This, token) \
    ((This)->lpVtbl->remove_DisableScannerRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_SetActiveSymbologiesRequested(This, handler, token) \
    ((This)->lpVtbl->add_SetActiveSymbologiesRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_SetActiveSymbologiesRequested(This, token) \
    ((This)->lpVtbl->remove_SetActiveSymbologiesRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_StartSoftwareTriggerRequested(This, handler, token) \
    ((This)->lpVtbl->add_StartSoftwareTriggerRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_StartSoftwareTriggerRequested(This, token) \
    ((This)->lpVtbl->remove_StartSoftwareTriggerRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_StopSoftwareTriggerRequested(This, handler, token) \
    ((This)->lpVtbl->add_StopSoftwareTriggerRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_StopSoftwareTriggerRequested(This, token) \
    ((This)->lpVtbl->remove_StopSoftwareTriggerRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_GetBarcodeSymbologyAttributesRequested(This, handler, token) \
    ((This)->lpVtbl->add_GetBarcodeSymbologyAttributesRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_GetBarcodeSymbologyAttributesRequested(This, token) \
    ((This)->lpVtbl->remove_GetBarcodeSymbologyAttributesRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_SetBarcodeSymbologyAttributesRequested(This, handler, token) \
    ((This)->lpVtbl->add_SetBarcodeSymbologyAttributesRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_SetBarcodeSymbologyAttributesRequested(This, token) \
    ((This)->lpVtbl->remove_SetBarcodeSymbologyAttributesRequested(This, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_add_HideVideoPreviewRequested(This, handler, token) \
    ((This)->lpVtbl->add_HideVideoPreviewRequested(This, handler, token))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_remove_HideVideoPreviewRequested(This, token) \
    ((This)->lpVtbl->remove_HideVideoPreviewRequested(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerProviderConnection2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateFrameReaderAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFrameReaderWithFormatAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat preferredFormat,
        __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFrameReaderWithFormatAndSizeAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat preferredFormat,
        struct __x_ABI_CWindows_CGraphics_CImaging_CBitmapSize preferredSize,
        __FIAsyncOperation_1_Windows__CDevices__CPointOfService__CProvider__CBarcodeScannerFrameReader** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_CreateFrameReaderAsync(This, operation) \
    ((This)->lpVtbl->CreateFrameReaderAsync(This, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_CreateFrameReaderWithFormatAsync(This, preferredFormat, operation) \
    ((This)->lpVtbl->CreateFrameReaderWithFormatAsync(This, preferredFormat, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_CreateFrameReaderWithFormatAndSizeAsync(This, preferredFormat, preferredSize, operation) \
    ((This)->lpVtbl->CreateFrameReaderWithFormatAndSizeAsync(This, preferredFormat, preferredSize, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerProviderTriggerDetails[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetailsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Connection)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderConnection** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetailsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetailsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_get_Connection(This, value) \
    ((This)->lpVtbl->get_Connection(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerProviderTriggerDetails_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetActiveSymbologiesRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Symbologies)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        __FIVectorView_1_UINT32** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_get_Symbologies(This, value) \
    ((This)->lpVtbl->get_Symbologies(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetActiveSymbologiesRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetActiveSymbologiesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetActiveSymbologiesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Symbology)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Attributes)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes** value);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_get_Symbology(This, value) \
    ((This)->lpVtbl->get_Symbology(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_get_Attributes(This, value) \
    ((This)->lpVtbl->get_Attributes(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetSymbologyAttributesRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerSetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerSetSymbologyAttributesRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStartSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStartSoftwareTriggerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStartSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStartSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStopSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportCompletedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
    HRESULT (STDMETHODCALLTYPE* ReportFailedAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_ReportCompletedAsync(This, result) \
    ((This)->lpVtbl->ReportCompletedAsync(This, result))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_ReportFailedAsync(This, result) \
    ((This)->lpVtbl->ReportFailedAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStopSoftwareTriggerRequest2[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This,
        INT32 reason,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
    HRESULT (STDMETHODCALLTYPE* ReportFailedWithFailedReasonAndDescriptionAsync)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2* This,
        INT32 reason,
        HSTRING failedReasonDescription,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2Vtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_ReportFailedWithFailedReasonAsync(This, reason, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAsync(This, reason, operation))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation) \
    ((This)->lpVtbl->ReportFailedWithFailedReasonAndDescriptionAsync(This, reason, failedReasonDescription, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerStopSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Request)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequest** value);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** result);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_get_Request(This, value) \
    ((This)->lpVtbl->get_Request(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_GetDeferral(This, result) \
    ((This)->lpVtbl->GetDeferral(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerStopSoftwareTriggerRequestEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeScannerVideoFrame[] = L"Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrameVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Format)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        enum __x_ABI_CWindows_CGraphics_CImaging_CBitmapPixelFormat* value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_PixelData)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame* This,
        __x_ABI_CWindows_CStorage_CStreams_CIBuffer** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrameVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrameVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_get_Format(This, value) \
    ((This)->lpVtbl->get_Format(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_get_PixelData(This, value) \
    ((This)->lpVtbl->get_PixelData(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeScannerVideoFrame_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Devices_PointOfService_Provider_IBarcodeSymbologyAttributesBuilder[] = L"Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder";
typedef struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsCheckDigitValidationSupported)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCheckDigitValidationSupported)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsCheckDigitTransmissionSupported)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsCheckDigitTransmissionSupported)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IsDecodeLengthSupported)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsDecodeLengthSupported)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* CreateAttributes)(__x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder* This,
        __x_ABI_CWindows_CDevices_CPointOfService_CIBarcodeSymbologyAttributes** value);

    END_INTERFACE
} __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilderVtbl;

interface __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder
{
    CONST_VTBL struct __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_get_IsCheckDigitValidationSupported(This, value) \
    ((This)->lpVtbl->get_IsCheckDigitValidationSupported(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_put_IsCheckDigitValidationSupported(This, value) \
    ((This)->lpVtbl->put_IsCheckDigitValidationSupported(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_get_IsCheckDigitTransmissionSupported(This, value) \
    ((This)->lpVtbl->get_IsCheckDigitTransmissionSupported(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_put_IsCheckDigitTransmissionSupported(This, value) \
    ((This)->lpVtbl->put_IsCheckDigitTransmissionSupported(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_get_IsDecodeLengthSupported(This, value) \
    ((This)->lpVtbl->get_IsDecodeLengthSupported(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_put_IsDecodeLengthSupported(This, value) \
    ((This)->lpVtbl->put_IsDecodeLengthSupported(This, value))

#define __x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_CreateAttributes(This, value) \
    ((This)->lpVtbl->CreateAttributes(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder;
#endif /* !defined(____x_ABI_CWindows_CDevices_CPointOfService_CProvider_CIBarcodeSymbologyAttributesBuilder_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerDisableScannerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerDisableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerDisableScannerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerEnableScannerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerEnableScannerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerEnableScannerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReader ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReader_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReader[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerFrameReaderFrameArrivedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReaderFrameArrivedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReaderFrameArrivedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerFrameReaderFrameArrivedEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerFrameReaderFrameArrivedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerGetSymbologyAttributesRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerGetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerGetSymbologyAttributesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerHideVideoPreviewRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerHideVideoPreviewRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerHideVideoPreviewRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderConnection2
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderConnection_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderConnection_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderConnection[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerProviderConnection";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerProviderTriggerDetails ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderTriggerDetails_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderTriggerDetails_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerProviderTriggerDetails[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerProviderTriggerDetails";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetActiveSymbologiesRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetActiveSymbologiesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetActiveSymbologiesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerSetSymbologyAttributesRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerSetSymbologyAttributesRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerSetSymbologyAttributesRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStartSoftwareTriggerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStartSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStartSoftwareTriggerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest ** Default Interface **
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequest2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequest_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequest[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerStopSoftwareTriggerRequestEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequestEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequestEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerStopSoftwareTriggerRequestEventArgs[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerStopSoftwareTriggerRequestEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeScannerVideoFrame ** Default Interface **
 *    Windows.Foundation.IClosable
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerVideoFrame_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeScannerVideoFrame_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeScannerVideoFrame[] = L"Windows.Devices.PointOfService.Provider.BarcodeScannerVideoFrame";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Devices.PointOfService.Provider.IBarcodeSymbologyAttributesBuilder ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeSymbologyAttributesBuilder_DEFINED
#define RUNTIMECLASS_Windows_Devices_PointOfService_Provider_BarcodeSymbologyAttributesBuilder_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Devices_PointOfService_Provider_BarcodeSymbologyAttributesBuilder[] = L"Windows.Devices.PointOfService.Provider.BarcodeSymbologyAttributesBuilder";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Edevices2Epointofservice2Eprovider_p_h__

#endif // __windows2Edevices2Epointofservice2Eprovider_h__
