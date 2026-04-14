
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
#ifndef __windows2Emedia2Eprotection2Eplayready_h__
#define __windows2Emedia2Eprotection2Eplayready_h__
#ifndef __windows2Emedia2Eprotection2Eplayready_p_h__
#define __windows2Emedia2Eprotection2Eplayready_p_h__


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

#if !defined(WINDOWS_MEDIA_PROTECTION_PROTECTIONRENEWALCONTRACT_VERSION)
#define WINDOWS_MEDIA_PROTECTION_PROTECTIONRENEWALCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_MEDIA_PROTECTION_PROTECTIONRENEWALCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.Media.Core.h"
#include "Windows.Media.Protection.h"
#include "Windows.Storage.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDClient;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient ABI::Windows::Media::Protection::PlayReady::INDClient

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDClientFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory ABI::Windows::Media::Protection::PlayReady::INDClientFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDClosedCaptionDataReceivedEventArgs;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs ABI::Windows::Media::Protection::PlayReady::INDClosedCaptionDataReceivedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDCustomData;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData ABI::Windows::Media::Protection::PlayReady::INDCustomData

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDCustomDataFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory ABI::Windows::Media::Protection::PlayReady::INDCustomDataFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDDownloadEngine;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine ABI::Windows::Media::Protection::PlayReady::INDDownloadEngine

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDDownloadEngineNotifier;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier ABI::Windows::Media::Protection::PlayReady::INDDownloadEngineNotifier

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDLicenseFetchCompletedEventArgs;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchCompletedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDLicenseFetchDescriptor;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptor

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDLicenseFetchDescriptorFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptorFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDLicenseFetchResult;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDMessenger;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger ABI::Windows::Media::Protection::PlayReady::INDMessenger

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDProximityDetectionCompletedEventArgs;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs ABI::Windows::Media::Protection::PlayReady::INDProximityDetectionCompletedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDRegistrationCompletedEventArgs;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs ABI::Windows::Media::Protection::PlayReady::INDRegistrationCompletedEventArgs

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDSendResult;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult ABI::Windows::Media::Protection::PlayReady::INDSendResult

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDStartResult;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult ABI::Windows::Media::Protection::PlayReady::INDStartResult

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDStorageFileHelper;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper ABI::Windows::Media::Protection::PlayReady::INDStorageFileHelper

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDStreamParser;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser ABI::Windows::Media::Protection::PlayReady::INDStreamParser

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDStreamParserNotifier;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier ABI::Windows::Media::Protection::PlayReady::INDStreamParserNotifier

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDTCPMessengerFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory ABI::Windows::Media::Protection::PlayReady::INDTCPMessengerFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface INDTransmitterProperties;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties ABI::Windows::Media::Protection::PlayReady::INDTransmitterProperties

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyContentHeader;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyContentHeader2;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2 ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader2

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyContentHeaderFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeaderFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyContentHeaderFactory2;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2 ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeaderFactory2

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyContentResolver;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentResolver

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyDomain;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyDomainIterableFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomainIterableFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyDomainJoinServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomainJoinServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyDomainLeaveServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomainLeaveServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyITADataGenerator;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator ABI::Windows::Media::Protection::PlayReady::IPlayReadyITADataGenerator

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyIndividualizationServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyIndividualizationServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicense;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicense2;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2 ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense2

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseAcquisitionServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseAcquisitionServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseAcquisitionServiceRequest2;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2 ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseAcquisitionServiceRequest2

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseAcquisitionServiceRequest3;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3 ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseAcquisitionServiceRequest3

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseIterableFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseIterableFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseManagement;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseManagement

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseSession;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseSession

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseSession2;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2 ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseSession2

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyLicenseSessionFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseSessionFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyMeteringReportServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyMeteringReportServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyRevocationServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyRevocationServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadySecureStopIterableFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopIterableFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadySecureStopServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadySecureStopServiceRequestFactory;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequestFactory

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest ABI::Windows::Media::Protection::PlayReady::IPlayReadyServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadySoapMessage;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage ABI::Windows::Media::Protection::PlayReady::IPlayReadySoapMessage

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyStatics;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics ABI::Windows::Media::Protection::PlayReady::IPlayReadyStatics

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyStatics2;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2 ABI::Windows::Media::Protection::PlayReady::IPlayReadyStatics2

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyStatics3;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3 ABI::Windows::Media::Protection::PlayReady::IPlayReadyStatics3

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyStatics4;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4 ABI::Windows::Media::Protection::PlayReady::IPlayReadyStatics4

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    interface IPlayReadyStatics5;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5 ABI::Windows::Media::Protection::PlayReady::IPlayReadyStatics5

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("9b9d0c68-3ad2-5b01-8dc7-6f7fc1eed6f6"))
IAsyncOperation<ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult*> : IAsyncOperation_impl<ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Protection.PlayReady.INDLicenseFetchResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult*> __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f72fdf87-9055-58d8-96ab-2cc04d06ccd7"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Protection.PlayReady.INDLicenseFetchResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6c3788a8-0e6d-5e29-9ad1-d88a65eb500b"))
IAsyncOperation<ABI::Windows::Media::Protection::PlayReady::INDSendResult*> : IAsyncOperation_impl<ABI::Windows::Media::Protection::PlayReady::INDSendResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Protection.PlayReady.INDSendResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Protection::PlayReady::INDSendResult*> __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c6c485d0-fdab-5142-b079-97af0567f0b6"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Protection::PlayReady::INDSendResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Media::Protection::PlayReady::INDSendResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Protection.PlayReady.INDSendResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Protection::PlayReady::INDSendResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("01fab63e-6aab-54a2-80f7-dbed22f58d56"))
IAsyncOperation<ABI::Windows::Media::Protection::PlayReady::INDStartResult*> : IAsyncOperation_impl<ABI::Windows::Media::Protection::PlayReady::INDStartResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.Protection.PlayReady.INDStartResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::Protection::PlayReady::INDStartResult*> __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_t;
#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1e4e3760-b22b-5f0a-9058-475aff310db5"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::Protection::PlayReady::INDStartResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Media::Protection::PlayReady::INDStartResult*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.Protection.PlayReady.INDStartResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::Protection::PlayReady::INDStartResult*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
        namespace Media {
            namespace Core {
                class AudioStreamDescriptor;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IAudioStreamDescriptor;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor ABI::Windows::Media::Core::IAudioStreamDescriptor

#endif // ____x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#define DEF___FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a61a11cd-b32e-518b-a6a7-5472cbe00e83"))
IIterator<ABI::Windows::Media::Core::AudioStreamDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::AudioStreamDescriptor*, ABI::Windows::Media::Core::IAudioStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Core.AudioStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Core::AudioStreamDescriptor*> __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t;
#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#define DEF___FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a3e2c972-a171-5b94-8389-e983ebc3f3b9"))
IIterable<ABI::Windows::Media::Core::AudioStreamDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::AudioStreamDescriptor*, ABI::Windows::Media::Core::IAudioStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Core.AudioStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Core::AudioStreamDescriptor*> __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t;
#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                class VideoStreamDescriptor;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IVideoStreamDescriptor;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor ABI::Windows::Media::Core::IVideoStreamDescriptor

#endif // ____x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#define DEF___FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("da51ab3c-3c64-545c-a3f4-f9b055aaf7d9"))
IIterator<ABI::Windows::Media::Core::VideoStreamDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::VideoStreamDescriptor*, ABI::Windows::Media::Core::IVideoStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Core.VideoStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Core::VideoStreamDescriptor*> __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t;
#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#define DEF___FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3acbf03c-0a79-5823-aaa9-d88bc3f8f594"))
IIterable<ABI::Windows::Media::Core::VideoStreamDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::VideoStreamDescriptor*, ABI::Windows::Media::Core::IVideoStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Core.VideoStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Core::VideoStreamDescriptor*> __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t;
#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_USE
#define DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5c35e8a5-4ad7-5e70-bedf-91f5d5888d35"))
IIterator<ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain*> : IIterator_impl<ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadyDomain>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain*> __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_t;
#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_USE
#define DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("84e98f86-4bee-5f41-93a1-255887122d9f"))
IIterable<ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain*> : IIterable_impl<ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadyDomain>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Protection::PlayReady::IPlayReadyDomain*> __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_t;
#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_USE
#define DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a6d000a-ce26-541b-a158-8457409b2604"))
IIterator<ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense*> : IIterator_impl<ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadyLicense>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense*> __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_t;
#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_USE
#define DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("9f28f6b7-b5ea-5073-ba3d-8cb2f07291a1"))
IIterable<ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense*> : IIterable_impl<ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadyLicense>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicense*> __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_t;
#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_USE
#define DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("77b0419b-3f4b-5ef9-ae0b-881143b172c9"))
IIterator<ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest*> : IIterator_impl<ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest*> __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_t;
#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_USE
#define DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8d8ac279-b07d-5308-9c78-1c5c996ca03c"))
IIterable<ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest*> : IIterable_impl<ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest*> __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_t;
#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_USE */

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

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("340f4fed-1288-5b89-be7c-c355fe1ce4d9"))
IVectorView<ABI::Windows::Media::Core::AudioStreamDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::AudioStreamDescriptor*, ABI::Windows::Media::Core::IAudioStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Core.AudioStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Core::AudioStreamDescriptor*> __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t;
#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e965c8af-d211-52f0-838b-4637469da7af"))
IVectorView<ABI::Windows::Media::Core::VideoStreamDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::VideoStreamDescriptor*, ABI::Windows::Media::Core::IVideoStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Core.VideoStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Core::VideoStreamDescriptor*> __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t;
#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE */

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

#ifndef DEF___FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#define DEF___FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("45afc129-988c-5f1e-9c17-6e34b917cd1b"))
IVector<ABI::Windows::Media::Core::AudioStreamDescriptor*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::AudioStreamDescriptor*, ABI::Windows::Media::Core::IAudioStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Core.AudioStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Core::AudioStreamDescriptor*> __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t;
#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#define DEF___FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1fb064b3-636c-5988-9c97-02a9b76150f6"))
IVector<ABI::Windows::Media::Core::VideoStreamDescriptor*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::VideoStreamDescriptor*, ABI::Windows::Media::Core::IVideoStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Core.VideoStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Core::VideoStreamDescriptor*> __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t;
#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_USE */

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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class NDClient;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d20f3387-b3f5-5010-9b0b-e851ae84940d"))
ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClient*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Protection.PlayReady.NDClient, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, IInspectable*> __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("36ec9e44-25f6-5d3d-87ee-8ad10a335c83"))
ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClosedCaptionDataReceivedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClient*>, ABI::Windows::Media::Protection::PlayReady::INDClosedCaptionDataReceivedEventArgs*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Protection.PlayReady.NDClient, Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClosedCaptionDataReceivedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a5401278-e2c1-5354-858f-6f32cbdfbea7"))
ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClient*>, ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchCompletedEventArgs*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Protection.PlayReady.NDClient, Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchCompletedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b545268a-c7ce-51f8-bddc-029d2c66b66b"))
ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDProximityDetectionCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClient*>, ABI::Windows::Media::Protection::PlayReady::INDProximityDetectionCompletedEventArgs*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Protection.PlayReady.NDClient, Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDProximityDetectionCompletedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("46b3f056-c1a0-51c0-8521-cde222157eff"))
ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDRegistrationCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDClient*>, ABI::Windows::Media::Protection::PlayReady::INDRegistrationCompletedEventArgs*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Media.Protection.PlayReady.NDClient, Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Media::Protection::PlayReady::NDClient*, ABI::Windows::Media::Protection::PlayReady::INDRegistrationCompletedEventArgs*> __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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
            typedef struct TimeSpan TimeSpan;
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaStreamDescriptor;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor ABI::Windows::Media::Core::IMediaStreamDescriptor

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                class MediaStreamSample;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaStreamSample;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample ABI::Windows::Media::Core::IMediaStreamSample

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                class MediaStreamSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Core {
                interface IMediaStreamSource;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource ABI::Windows::Media::Core::IMediaStreamSource

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                interface IMediaProtectionServiceRequest;
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest ABI::Windows::Media::Protection::IMediaProtectionServiceRequest

#endif // ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                class MediaProtectionManager;
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                interface IMediaProtectionManager;
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager ABI::Windows::Media::Protection::IMediaProtectionManager

#endif // ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager_FWD_DEFINED__

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
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDCertificateFeature : int NDCertificateFeature;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDCertificatePlatformID : int NDCertificatePlatformID;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDCertificateType : int NDCertificateType;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDClosedCaptionFormat : int NDClosedCaptionFormat;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDContentIDType : int NDContentIDType;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDMediaStreamType : int NDMediaStreamType;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum NDProximityDetectionType : int NDProximityDetectionType;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum PlayReadyDecryptorSetup : int PlayReadyDecryptorSetup;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum PlayReadyEncryptionAlgorithm : int PlayReadyEncryptionAlgorithm;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum PlayReadyHardwareDRMFeatures : int PlayReadyHardwareDRMFeatures;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    typedef enum PlayReadyITADataFormat : int PlayReadyITADataFormat;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class NDCustomData;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class NDDownloadEngineNotifier;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class NDLicenseFetchDescriptor;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class NDStreamParserNotifier;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class NDTCPMessenger;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadyContentHeader;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadyDomainIterable;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadyLicenseIterable;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadyLicenseSession;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadySecureStopIterable;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadySecureStopServiceRequest;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    class PlayReadySoapMessage;
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDCertificateFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDCertificateFeature is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDCertificateFeature : int
                    {
                        NDCertificateFeature_Transmitter = 1,
                        NDCertificateFeature_Receiver = 2,
                        NDCertificateFeature_SharedCertificate = 3,
                        NDCertificateFeature_SecureClock = 4,
                        NDCertificateFeature_AntiRollBackClock = 5,
                        NDCertificateFeature_CRLS = 9,
                        NDCertificateFeature_PlayReady3Features = 13,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDCertificatePlatformID
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDCertificatePlatformID is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDCertificatePlatformID : int
                    {
                        NDCertificatePlatformID_Windows = 0,
                        NDCertificatePlatformID_OSX = 1,
                        NDCertificatePlatformID_WindowsOnARM = 2,
                        NDCertificatePlatformID_WindowsMobile7 = 5,
                        NDCertificatePlatformID_iOSOnARM = 6,
                        NDCertificatePlatformID_XBoxOnPPC = 7,
                        NDCertificatePlatformID_WindowsPhone8OnARM = 8,
                        NDCertificatePlatformID_WindowsPhone8OnX86 = 9,
                        NDCertificatePlatformID_XboxOne = 10,
                        NDCertificatePlatformID_AndroidOnARM = 11,
                        NDCertificatePlatformID_WindowsPhone81OnARM = 12,
                        NDCertificatePlatformID_WindowsPhone81OnX86 = 13,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDCertificateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDCertificateType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDCertificateType : int
                    {
                        NDCertificateType_Unknown = 0,
                        NDCertificateType_PC = 1,
                        NDCertificateType_Device = 2,
                        NDCertificateType_Domain = 3,
                        NDCertificateType_Issuer = 4,
                        NDCertificateType_CrlSigner = 5,
                        NDCertificateType_Service = 6,
                        NDCertificateType_Silverlight = 7,
                        NDCertificateType_Application = 8,
                        NDCertificateType_Metering = 9,
                        NDCertificateType_KeyFileSigner = 10,
                        NDCertificateType_Server = 11,
                        NDCertificateType_LicenseSigner = 12,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDClosedCaptionFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDClosedCaptionFormat is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDClosedCaptionFormat : int
                    {
                        NDClosedCaptionFormat_ATSC = 0,
                        NDClosedCaptionFormat_SCTE20 = 1,
                        NDClosedCaptionFormat_Unknown = 2,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDContentIDType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDContentIDType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDContentIDType : int
                    {
                        NDContentIDType_KeyID = 1,
                        NDContentIDType_PlayReadyObject = 2,
                        NDContentIDType_Custom = 3,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDMediaStreamType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDMediaStreamType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDMediaStreamType : int
                    {
                        NDMediaStreamType_Audio = 1,
                        NDMediaStreamType_Video = 2,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDProximityDetectionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDProximityDetectionType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDProximityDetectionType : int
                    {
                        NDProximityDetectionType_UDP = 1,
                        NDProximityDetectionType_TCP = 2,
                        NDProximityDetectionType_TransportAgnostic = 4,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDStartAsyncOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("NDStartAsyncOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    NDStartAsyncOptions : int
                    {
                        NDStartAsyncOptions_MutualAuthentication = 1,
                        NDStartAsyncOptions_WaitForLicenseDescriptor = 2,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum PlayReadyDecryptorSetup : int
                    {
                        PlayReadyDecryptorSetup_Uninitialized = 0,
                        PlayReadyDecryptorSetup_OnDemand = 1,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum PlayReadyEncryptionAlgorithm : int
                    {
                        PlayReadyEncryptionAlgorithm_Unprotected = 0,
                        PlayReadyEncryptionAlgorithm_Aes128Ctr = 1,
                        PlayReadyEncryptionAlgorithm_Cocktail = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        PlayReadyEncryptionAlgorithm_Aes128Cbc = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        PlayReadyEncryptionAlgorithm_Unspecified = 65535,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        PlayReadyEncryptionAlgorithm_Uninitialized = 2147483647,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum PlayReadyHardwareDRMFeatures : int
                    {
                        PlayReadyHardwareDRMFeatures_HardwareDRM = 1,
                        PlayReadyHardwareDRMFeatures_HEVC = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                        PlayReadyHardwareDRMFeatures_Aes128Cbc = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyITADataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    enum PlayReadyITADataFormat : int
                    {
                        PlayReadyITADataFormat_SerializedProperties = 0,
                        PlayReadyITADataFormat_SerializedProperties_WithContentProtectionWrapper = 1,
                    };
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDClient[] = L"Windows.Media.Protection.PlayReady.INDClient";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("3bd6781b-61b8-46e2-99a5-8abcb6b9f7d6")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDClient : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE add_RegistrationCompleted(
                            __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE remove_RegistrationCompleted(
                            EventRegistrationToken token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE add_ProximityDetectionCompleted(
                            __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE remove_ProximityDetectionCompleted(
                            EventRegistrationToken token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE add_LicenseFetchCompleted(
                            __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE remove_LicenseFetchCompleted(
                            EventRegistrationToken token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE add_ReRegistrationNeeded(
                            __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable* handler,
                            EventRegistrationToken* token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE remove_ReRegistrationNeeded(
                            EventRegistrationToken token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE add_ClosedCaptionDataReceived(
                            __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs* handler,
                            EventRegistrationToken* token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE remove_ClosedCaptionDataReceived(
                            EventRegistrationToken token
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE StartAsync(
                            ABI::Windows::Foundation::IUriRuntimeClass* contentUrl,
                            UINT32 startAsyncOptions,
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData* registrationCustomData,
                            ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptor* licenseFetchDescriptor,
                            __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE LicenseFetchAsync(
                            ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptor* licenseFetchDescriptor,
                            __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE ReRegistrationAsync(
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData* registrationCustomData,
                            ABI::Windows::Foundation::IAsyncAction** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE Close(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDClient = __uuidof(INDClient);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDClientFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDClientFactory[] = L"Windows.Media.Protection.PlayReady.INDClientFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("3e53dd62-fee8-451f-b0d4-f706cca3e037")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDClientFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDClientFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClientFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Media::Protection::PlayReady::INDDownloadEngine* downloadEngine,
                            ABI::Windows::Media::Protection::PlayReady::INDStreamParser* streamParser,
                            ABI::Windows::Media::Protection::PlayReady::INDMessenger* pMessenger,
                            ABI::Windows::Media::Protection::PlayReady::INDClient** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDClientFactory = __uuidof(INDClientFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDClosedCaptionDataReceivedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("4738d29f-c345-4649-8468-b8c5fc357190")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDClosedCaptionDataReceivedEventArgs : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ClosedCaptionDataFormat(
                            ABI::Windows::Media::Protection::PlayReady::NDClosedCaptionFormat* ccForamt
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_PresentationTimestamp(
                            INT64* presentationTimestamp
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ClosedCaptionData(
                            UINT32* ccDataBytesLength,
                            BYTE** ccDataBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDClosedCaptionDataReceivedEventArgs = __uuidof(INDClosedCaptionDataReceivedEventArgs);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDCustomData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDCustomData[] = L"Windows.Media.Protection.PlayReady.INDCustomData";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("f5cb0fdc-2d09-4f19-b5e1-76a0b3ee9267")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDCustomData : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_CustomDataTypeID(
                            UINT32* customDataTypeIDBytesLength,
                            BYTE** customDataTypeIDBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_CustomData(
                            UINT32* customDataBytesLength,
                            BYTE** customDataBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDCustomData = __uuidof(INDCustomData);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDCustomDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDCustomData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDCustomDataFactory[] = L"Windows.Media.Protection.PlayReady.INDCustomDataFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d65405ab-3424-4833-8c9a-af5fdeb22872")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDCustomDataFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDCustomDataFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDCustomDataFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            UINT32 customDataTypeIDBytesLength,
                            BYTE* customDataTypeIDBytes,
                            UINT32 customDataBytesLength,
                            BYTE* customDataBytes,
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDCustomDataFactory = __uuidof(INDCustomDataFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDDownloadEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDDownloadEngine[] = L"Windows.Media.Protection.PlayReady.INDDownloadEngine";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("2d223d65-c4b6-4438-8d46-b96e6d0fb21f")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDDownloadEngine : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE Open(
                            ABI::Windows::Foundation::IUriRuntimeClass* uri,
                            UINT32 sessionIDBytesLength,
                            BYTE* sessionIDBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE Pause(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE Resume(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE Close(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE Seek(
                            ABI::Windows::Foundation::TimeSpan startPosition
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_CanSeek(
                            boolean* canSeek
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_BufferFullMinThresholdInSamples(
                            UINT32* bufferFullMinThreshold
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_BufferFullMaxThresholdInSamples(
                            UINT32* bufferFullMaxThreshold
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_Notifier(
                            ABI::Windows::Media::Protection::PlayReady::INDDownloadEngineNotifier** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDDownloadEngine = __uuidof(INDDownloadEngine);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDDownloadEngineNotifier[] = L"Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d720b4d4-f4b8-4530-a809-9193a571e7fc")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDDownloadEngineNotifier : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnStreamOpened(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnPlayReadyObjectReceived(
                            UINT32 dataBytesLength,
                            BYTE* dataBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnContentIDReceived(
                            ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptor* licenseFetchDescriptor
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnDataReceived(
                            UINT32 dataBytesLength,
                            BYTE* dataBytes,
                            UINT32 bytesReceived
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnEndOfStream(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnNetworkError(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDDownloadEngineNotifier = __uuidof(INDDownloadEngineNotifier);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchCompletedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("1ee30a1a-11b2-4558-8865-e3a516922517")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDLicenseFetchCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDLicenseFetchCompletedEventArgs : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseCustomData(
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData** customData
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDLicenseFetchCompletedEventArgs = __uuidof(INDLicenseFetchCompletedEventArgs);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchDescriptor[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("5498d33a-e686-4935-a567-7ca77ad20fa4")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDLicenseFetchDescriptor : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ContentIDType(
                            ABI::Windows::Media::Protection::PlayReady::NDContentIDType* contentIDType
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ContentID(
                            UINT32* contentIDBytesLength,
                            BYTE** contentIDBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_LicenseFetchChallengeCustomData(
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData** licenseFetchChallengeCustomData
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE put_LicenseFetchChallengeCustomData(
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData* licenseFetchChallengeCustomData
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDLicenseFetchDescriptor = __uuidof(INDLicenseFetchDescriptor);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchDescriptorFactory[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d0031202-cfac-4f00-ae6a-97af80b848f2")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDLicenseFetchDescriptorFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDLicenseFetchDescriptorFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchDescriptorFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Media::Protection::PlayReady::NDContentIDType contentIDType,
                            UINT32 contentIDBytesLength,
                            BYTE* contentIDBytes,
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData* licenseFetchChallengeCustomData,
                            ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptor** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDLicenseFetchDescriptorFactory = __uuidof(INDLicenseFetchDescriptorFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchResult[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("21d39698-aa62-45ff-a5ff-8037e5433825")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDLicenseFetchResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDLicenseFetchResult : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDLicenseFetchResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseCustomData(
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData** customData
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDLicenseFetchResult = __uuidof(INDLicenseFetchResult);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDMessenger
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDMessenger[] = L"Windows.Media.Protection.PlayReady.INDMessenger";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d42df95d-a75b-47bf-8249-bc83820da38a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDMessenger : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE SendRegistrationRequestAsync(
                            UINT32 sessionIDBytesLength,
                            BYTE* sessionIDBytes,
                            UINT32 challengeDataBytesLength,
                            BYTE* challengeDataBytes,
                            __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE SendProximityDetectionStartAsync(
                            ABI::Windows::Media::Protection::PlayReady::NDProximityDetectionType pdType,
                            UINT32 transmitterChannelBytesLength,
                            BYTE* transmitterChannelBytes,
                            UINT32 sessionIDBytesLength,
                            BYTE* sessionIDBytes,
                            UINT32 challengeDataBytesLength,
                            BYTE* challengeDataBytes,
                            __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE SendProximityDetectionResponseAsync(
                            ABI::Windows::Media::Protection::PlayReady::NDProximityDetectionType pdType,
                            UINT32 transmitterChannelBytesLength,
                            BYTE* transmitterChannelBytes,
                            UINT32 sessionIDBytesLength,
                            BYTE* sessionIDBytes,
                            UINT32 responseDataBytesLength,
                            BYTE* responseDataBytes,
                            __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE SendLicenseFetchRequestAsync(
                            UINT32 sessionIDBytesLength,
                            BYTE* sessionIDBytes,
                            UINT32 challengeDataBytesLength,
                            BYTE* challengeDataBytes,
                            __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDMessenger = __uuidof(INDMessenger);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDProximityDetectionCompletedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("2a706328-da25-4f8c-9eb7-5d0fc3658bca")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDProximityDetectionCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDProximityDetectionCompletedEventArgs : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDProximityDetectionCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ProximityDetectionRetryCount(
                            UINT32* retryCount
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDProximityDetectionCompletedEventArgs = __uuidof(INDProximityDetectionCompletedEventArgs);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDRegistrationCompletedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("9e39b64d-ab5b-4905-acdc-787a77c6374d")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDRegistrationCompletedEventArgs : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseCustomData(
                            ABI::Windows::Media::Protection::PlayReady::INDCustomData** customData
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_TransmitterProperties(
                            ABI::Windows::Media::Protection::PlayReady::INDTransmitterProperties** transmitterProperties
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_TransmitterCertificateAccepted(
                            boolean* acceptpt
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE put_TransmitterCertificateAccepted(
                            boolean accept
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDRegistrationCompletedEventArgs = __uuidof(INDRegistrationCompletedEventArgs);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDSendResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDSendResult[] = L"Windows.Media.Protection.PlayReady.INDSendResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("e3685517-a584-479d-90b7-d689c7bf7c80")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDSendResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDSendResult : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDSendResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_Response(
                            UINT32* responseDataBytesLength,
                            BYTE** responseDataBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDSendResult = __uuidof(INDSendResult);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStartResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStartResult[] = L"Windows.Media.Protection.PlayReady.INDStartResult";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("79f6e96e-f50f-4015-8ba4-c2bc344ebd4e")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDStartResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDStartResult : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStartResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_MediaStreamSource(
                            ABI::Windows::Media::Core::IMediaStreamSource** mediaStreamSource
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDStartResult = __uuidof(INDStartResult);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStorageFileHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStorageFileHelper[] = L"Windows.Media.Protection.PlayReady.INDStorageFileHelper";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d8f0bef8-91d2-4d47-a3f9-eaff4edb729f")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDStorageFileHelper : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE GetFileURLs(
                            ABI::Windows::Storage::IStorageFile* file,
                            __FIVector_1_HSTRING** fileURLs
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDStorageFileHelper = __uuidof(INDStorageFileHelper);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStreamParser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStreamParser[] = L"Windows.Media.Protection.PlayReady.INDStreamParser";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("e0baa198-9796-41c9-8695-59437e67e66a")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDStreamParser : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE ParseData(
                            UINT32 dataBytesLength,
                            BYTE* dataBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE GetStreamInformation(
                            ABI::Windows::Media::Core::IMediaStreamDescriptor* descriptor,
                            ABI::Windows::Media::Protection::PlayReady::NDMediaStreamType* streamType,
                            UINT32* streamID
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE BeginOfStream(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE EndOfStream(void) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_Notifier(
                            ABI::Windows::Media::Protection::PlayReady::INDStreamParserNotifier** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDStreamParser = __uuidof(INDStreamParser);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStreamParserNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStreamParserNotifier[] = L"Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("c167acd0-2ce6-426c-ace5-5e9275fea715")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDStreamParserNotifier : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnContentIDReceived(
                            ABI::Windows::Media::Protection::PlayReady::INDLicenseFetchDescriptor* licenseFetchDescriptor
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnMediaStreamDescriptorCreated(
                            __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* audioStreamDescriptors,
                            __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* videoStreamDescriptors
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnSampleParsed(
                            UINT32 streamID,
                            ABI::Windows::Media::Protection::PlayReady::NDMediaStreamType streamType,
                            ABI::Windows::Media::Core::IMediaStreamSample* streamSample,
                            INT64 pts,
                            ABI::Windows::Media::Protection::PlayReady::NDClosedCaptionFormat ccFormat,
                            UINT32 ccDataBytesLength,
                            BYTE* ccDataBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE OnBeginSetupDecryptor(
                            ABI::Windows::Media::Core::IMediaStreamDescriptor* descriptor,
                            GUID keyID,
                            UINT32 proBytesLength,
                            BYTE* proBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDStreamParserNotifier = __uuidof(INDStreamParserNotifier);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDTCPMessengerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDTCPMessenger
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDTCPMessengerFactory[] = L"Windows.Media.Protection.PlayReady.INDTCPMessengerFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("7dd85cfe-1b99-4f68-8f82-8177f7cedf2b")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDTCPMessengerFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDTCPMessengerFactory : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTCPMessengerFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            HSTRING remoteHostName,
                            UINT32 remoteHostPort,
                            ABI::Windows::Media::Protection::PlayReady::INDMessenger** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDTCPMessengerFactory = __uuidof(INDTCPMessengerFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDTransmitterProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDTransmitterProperties[] = L"Windows.Media.Protection.PlayReady.INDTransmitterProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("e536af23-ac4f-4adc-8c66-4ff7c2702dd6")
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                    INDTransmitterProperties : public IInspectable
                    {
                    public:
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_CertificateType(
                            ABI::Windows::Media::Protection::PlayReady::NDCertificateType* type
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_PlatformIdentifier(
                            ABI::Windows::Media::Protection::PlayReady::NDCertificatePlatformID* identifier
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_SupportedFeatures(
                            UINT32* featureSetsLength,
                            ABI::Windows::Media::Protection::PlayReady::NDCertificateFeature** featureSets
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_SecurityLevel(
                            UINT32* level
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_SecurityVersion(
                            UINT32* securityVersion
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ExpirationDate(
                            ABI::Windows::Foundation::DateTime* expirationDate
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ClientID(
                            UINT32* clientIDBytesLength,
                            BYTE** clientIDBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ModelDigest(
                            UINT32* modelDigestBytesLength,
                            BYTE** modelDigestBytes
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ModelManufacturerName(
                            HSTRING* modelManufacturerName
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ModelName(
                            HSTRING* modelName
                            ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
                        virtual HRESULT STDMETHODCALLTYPE get_ModelNumber(
                            HSTRING* modelNumber
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_INDTransmitterProperties = __uuidof(INDTransmitterProperties);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeader[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeader";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("9a438a6a-7f4c-452e-88bd-0148c6387a2c")
                    IPlayReadyContentHeader : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_KeyId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyIdString(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LicenseAcquisitionUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LicenseAcquisitionUserInterfaceUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainServiceId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_EncryptionType(
                            ABI::Windows::Media::Protection::PlayReady::PlayReadyEncryptionAlgorithm* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_CustomAttributes(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DecryptorSetup(
                            ABI::Windows::Media::Protection::PlayReady::PlayReadyDecryptorSetup* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetSerializedHeader(
                            UINT32* headerBytesLength,
                            BYTE** headerBytes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HeaderWithEmbeddedUpdates(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyContentHeader = __uuidof(IPlayReadyContentHeader);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeader2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("359c79f4-2180-498c-965b-e754d875eab2")
                    IPlayReadyContentHeader2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_KeyIds(
                            UINT32* contentKeyIdsLength,
                            GUID** contentKeyIds
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_KeyIdStrings(
                            UINT32* contentKeyIdStringsLength,
                            HSTRING** contentKeyIdStrings
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyContentHeader2 = __uuidof(IPlayReadyContentHeader2);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeaderFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("cb97c8ff-b758-4776-bf01-217a8b510b2c")
                    IPlayReadyContentHeaderFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromWindowsMediaDrmHeader(
                            UINT32 headerBytesLength,
                            BYTE* headerBytes,
                            ABI::Windows::Foundation::IUriRuntimeClass* licenseAcquisitionUrl,
                            ABI::Windows::Foundation::IUriRuntimeClass* licenseAcquisitionUserInterfaceUrl,
                            HSTRING customAttributes,
                            GUID domainServiceId,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader** instance
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromComponents(
                            GUID contentKeyId,
                            HSTRING contentKeyIdString,
                            ABI::Windows::Media::Protection::PlayReady::PlayReadyEncryptionAlgorithm contentEncryptionAlgorithm,
                            ABI::Windows::Foundation::IUriRuntimeClass* licenseAcquisitionUrl,
                            ABI::Windows::Foundation::IUriRuntimeClass* licenseAcquisitionUserInterfaceUrl,
                            HSTRING customAttributes,
                            GUID domainServiceId,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader** instance
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromPlayReadyHeader(
                            UINT32 headerBytesLength,
                            BYTE* headerBytes,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyContentHeaderFactory = __uuidof(IPlayReadyContentHeaderFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeaderFactory2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d1239cf5-ae6d-4778-97fd-6e3a2eeadbeb")
                    IPlayReadyContentHeaderFactory2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromComponents2(
                            UINT32 dwFlags,
                            UINT32 contentKeyIdsLength,
                            GUID* contentKeyIds,
                            UINT32 contentKeyIdStringsLength,
                            HSTRING* contentKeyIdStrings,
                            ABI::Windows::Media::Protection::PlayReady::PlayReadyEncryptionAlgorithm contentEncryptionAlgorithm,
                            ABI::Windows::Foundation::IUriRuntimeClass* licenseAcquisitionUrl,
                            ABI::Windows::Foundation::IUriRuntimeClass* licenseAcquisitionUserInterfaceUrl,
                            HSTRING customAttributes,
                            GUID domainServiceId,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyContentHeaderFactory2 = __uuidof(IPlayReadyContentHeaderFactory2);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentResolver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentResolver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentResolver[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentResolver";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("fbfd2523-906d-4982-a6b8-6849565a7ce8")
                    IPlayReadyContentResolver : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE ServiceRequest(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader* contentHeader,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyServiceRequest** serviceRequest
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyContentResolver = __uuidof(IPlayReadyContentResolver);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomain[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomain";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("adcc93ac-97e6-43ef-95e4-d7868f3b16a9")
                    IPlayReadyDomain : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_AccountId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ServiceId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Revision(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_FriendlyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainJoinUrl(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyDomain = __uuidof(IPlayReadyDomain);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyDomainIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomainIterableFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("4df384ee-3121-4df3-a5e8-d0c24c0500fc")
                    IPlayReadyDomainIterableFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            GUID domainAccountId,
                            __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain** domainIterable
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyDomainIterableFactory = __uuidof(IPlayReadyDomainIterableFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomainJoinServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("171b4a5a-405f-4739-b040-67b9f0c38758")
                    IPlayReadyDomainJoinServiceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DomainAccountId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DomainAccountId(
                            GUID value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainFriendlyName(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DomainFriendlyName(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainServiceId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DomainServiceId(
                            GUID value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyDomainJoinServiceRequest = __uuidof(IPlayReadyDomainJoinServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomainLeaveServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("062d58be-97ad-4917-aa03-46d4c252d464")
                    IPlayReadyDomainLeaveServiceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DomainAccountId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DomainAccountId(
                            GUID value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainServiceId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DomainServiceId(
                            GUID value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyDomainLeaveServiceRequest = __uuidof(IPlayReadyDomainLeaveServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyITADataGenerator[] = L"Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("24446b8e-10b9-4530-b25b-901a8029a9b2")
                    IPlayReadyITADataGenerator : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GenerateData(
                            GUID guidCPSystemId,
                            UINT32 countOfStreams,
                            ABI::Windows::Foundation::Collections::IPropertySet* configuration,
                            ABI::Windows::Media::Protection::PlayReady::PlayReadyITADataFormat format,
                            UINT32* dataBytesLength,
                            BYTE** dataBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyITADataGenerator = __uuidof(IPlayReadyITADataGenerator);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyIndividualizationServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("21f5a86b-008c-4611-ab2f-aaa6c69f0e24")
                    IPlayReadyIndividualizationServiceRequest : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyIndividualizationServiceRequest = __uuidof(IPlayReadyIndividualizationServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicense[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicense";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("ee474c4e-fa3c-414d-a9f2-3ffc1ef832d4")
                    IPlayReadyLicense : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_FullyEvaluated(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UsableForPlay(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExpirationDate(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExpireAfterFirstPlay(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainAccountID(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ChainDepth(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GetKIDAtChainDepth(
                            UINT32 chainDepth,
                            GUID* kid
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicense = __uuidof(IPlayReadyLicense);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicense2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicense
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicense
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicense2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicense2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("30f4e7a7-d8e3-48a0-bcda-ff9f40530436")
                    IPlayReadyLicense2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SecureStopId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_SecurityLevel(
                            UINT32* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_InMemoryOnly(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ExpiresInRealTime(
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicense2 = __uuidof(IPlayReadyLicense2);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseAcquisitionServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("5d85ff45-3e9f-4f48-93e1-9530c8d58c3e")
                    IPlayReadyLicenseAcquisitionServiceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_ContentHeader(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ContentHeader(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainServiceId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_DomainServiceId(
                            GUID value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseAcquisitionServiceRequest = __uuidof(IPlayReadyLicenseAcquisitionServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseAcquisitionServiceRequest2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("b7fa5eb5-fe0c-b225-bc60-5a9edd32ceb5")
                    IPlayReadyLicenseAcquisitionServiceRequest2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SessionId(
                            GUID* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseAcquisitionServiceRequest2 = __uuidof(IPlayReadyLicenseAcquisitionServiceRequest2);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseAcquisitionServiceRequest3[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("394e5f4d-7f75-430d-b2e7-7f75f34b2d75")
                    IPlayReadyLicenseAcquisitionServiceRequest3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateLicenseIterable(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader* contentHeader,
                            boolean fullyEvaluated,
                            __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseAcquisitionServiceRequest3 = __uuidof(IPlayReadyLicenseAcquisitionServiceRequest3);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseIterableFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("d4179f08-0837-4978-8e68-be4293c8d7a6")
                    IPlayReadyLicenseIterableFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader* contentHeader,
                            boolean fullyEvaluated,
                            __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseIterableFactory = __uuidof(IPlayReadyLicenseIterableFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseManagement[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("aaeb2141-0957-4405-b892-8bf3ec5dadd9")
                    IPlayReadyLicenseManagement : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE DeleteLicenses(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader* contentHeader,
                            ABI::Windows::Foundation::IAsyncAction** operation
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseManagement = __uuidof(IPlayReadyLicenseManagement);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseSession[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("a1723a39-87fa-4fdd-abbb-a9720e845259")
                    IPlayReadyLicenseSession : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateLAServiceRequest(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseAcquisitionServiceRequest** serviceRequest
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ConfigureMediaProtectionManager(
                            ABI::Windows::Media::Protection::IMediaProtectionManager* mpm
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseSession = __uuidof(IPlayReadyLicenseSession);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseSession2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("4909be3a-3aed-4656-8ad7-ee0fd7799510")
                    IPlayReadyLicenseSession2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateLicenseIterable(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyContentHeader* contentHeader,
                            boolean fullyEvaluated,
                            __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** licenseIterable
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseSession2 = __uuidof(IPlayReadyLicenseSession2);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseSessionFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("62492699-6527-429e-98be-48d798ac2739")
                    IPlayReadyLicenseSessionFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            ABI::Windows::Foundation::Collections::IPropertySet* configuration,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyLicenseSession** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyLicenseSessionFactory = __uuidof(IPlayReadyLicenseSessionFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyMeteringReportServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("c12b231c-0ecd-4f11-a185-1e24a4a67fb7")
                    IPlayReadyMeteringReportServiceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_MeteringCertificate(
                            UINT32* meteringCertBytesLength,
                            BYTE** meteringCertBytes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_MeteringCertificate(
                            UINT32 meteringCertBytesLength,
                            BYTE* meteringCertBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyMeteringReportServiceRequest = __uuidof(IPlayReadyMeteringReportServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyRevocationServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("543d66ac-faf0-4560-84a5-0e4acec939e4")
                    IPlayReadyRevocationServiceRequest : public IInspectable
                    {
                    public:
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyRevocationServiceRequest = __uuidof(IPlayReadyRevocationServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySecureStopIterableFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("5f1f0165-4214-4d9e-81eb-e89f9d294aee")
                    IPlayReadySecureStopIterableFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            UINT32 publisherCertBytesLength,
                            BYTE* publisherCertBytes,
                            __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadySecureStopIterableFactory = __uuidof(IPlayReadySecureStopIterableFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySecureStopServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("b5501ee5-01bf-4401-9677-05630a6a4cc8")
                    IPlayReadySecureStopServiceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SessionID(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_StartTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_UpdateTime(
                            ABI::Windows::Foundation::DateTime* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Stopped(
                            boolean* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PublisherCertificate(
                            UINT32* publisherCertBytesLength,
                            BYTE** publisherCertBytes
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadySecureStopServiceRequest = __uuidof(IPlayReadySecureStopServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySecureStopServiceRequestFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("0e448ac9-e67e-494e-9f49-6285438c76cf")
                    IPlayReadySecureStopServiceRequestFactory : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                            UINT32 publisherCertBytesLength,
                            BYTE* publisherCertBytes,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest** instance
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CreateInstanceFromSessionID(
                            GUID sessionID,
                            UINT32 publisherCertBytesLength,
                            BYTE* publisherCertBytes,
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadySecureStopServiceRequest** instance
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadySecureStopServiceRequestFactory = __uuidof(IPlayReadySecureStopServiceRequestFactory);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("8bad2836-a703-45a6-a180-76f3565aa725")
                    IPlayReadyServiceRequest : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ResponseCustomData(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ChallengeCustomData(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE put_ChallengeCustomData(
                            HSTRING value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE BeginServiceRequest(
                            ABI::Windows::Foundation::IAsyncAction** action
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE NextServiceRequest(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadyServiceRequest** serviceRequest
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE GenerateManualEnablingChallenge(
                            ABI::Windows::Media::Protection::PlayReady::IPlayReadySoapMessage** challengeMessage
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ProcessManualEnablingResponse(
                            UINT32 responseBytesLength,
                            BYTE* responseBytes,
                            HRESULT* result
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyServiceRequest = __uuidof(IPlayReadyServiceRequest);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySoapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadySoapMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySoapMessage[] = L"Windows.Media.Protection.PlayReady.IPlayReadySoapMessage";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("b659fcb5-ce41-41ba-8a0d-61df5fffa139")
                    IPlayReadySoapMessage : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE GetMessageBody(
                            UINT32* messageBodyBytesLength,
                            BYTE** messageBodyBytes
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MessageHeaders(
                            ABI::Windows::Foundation::Collections::IPropertySet** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_Uri(
                            ABI::Windows::Foundation::IUriRuntimeClass** messageUri
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadySoapMessage = __uuidof(IPlayReadySoapMessage);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("5e69c00d-247c-469a-8f31-5c1a1571d9c6")
                    IPlayReadyStatics : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_DomainJoinServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_DomainLeaveServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_IndividualizationServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_LicenseAcquirerServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MeteringReportServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_RevocationServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_MediaProtectionSystemId(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_PlayReadySecurityVersion(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyStatics = __uuidof(IPlayReadyStatics);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("1f8d6a92-5f9a-423e-9466-b33969af7a3d")
                    IPlayReadyStatics2 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_PlayReadyCertificateSecurityLevel(
                            UINT32* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyStatics2 = __uuidof(IPlayReadyStatics2);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics3[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("3fa33f71-2dd3-4bed-ae49-f7148e63e710")
                    IPlayReadyStatics3 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_SecureStopServiceRequestType(
                            GUID* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE CheckSupportedHardware(
                            ABI::Windows::Media::Protection::PlayReady::PlayReadyHardwareDRMFeatures hwdrmFeature,
                            boolean* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyStatics3 = __uuidof(IPlayReadyStatics3);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics3
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics4[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics4";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("50a91300-d824-4231-9d5e-78ef8844c7d7")
                    IPlayReadyStatics4 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_InputTrustAuthorityToCreate(
                            HSTRING* value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_ProtectionSystemId(
                            GUID* value
                            ) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyStatics4 = __uuidof(IPlayReadyStatics4);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics4
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics3
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics5[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics5";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace Protection {
                namespace PlayReady {
                    MIDL_INTERFACE("230a7075-dfa0-4f8e-a779-cefea9c6824b")
                    IPlayReadyStatics5 : public IInspectable
                    {
                    public:
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareDRMDisabledAtTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE get_HardwareDRMDisabledUntilTime(
                            __FIReference_1_Windows__CFoundation__CDateTime** value
                            ) = 0;
                        virtual HRESULT STDMETHODCALLTYPE ResetHardwareDRMDisabled(void) = 0;
                    };

                    MIDL_CONST_ID IID& IID_IPlayReadyStatics5 = __uuidof(IPlayReadyStatics5);
                } /* PlayReady */
            } /* Protection */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDClientFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDClient ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDClient_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDClient_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDClient[] = L"Windows.Media.Protection.PlayReady.NDClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDCustomData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDCustomDataFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDCustomData ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDCustomData_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDCustomData_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDCustomData[] = L"Windows.Media.Protection.PlayReady.NDCustomData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDDownloadEngineNotifier_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDDownloadEngineNotifier_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDDownloadEngineNotifier[] = L"Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDLicenseFetchDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDLicenseFetchDescriptor_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDLicenseFetchDescriptor[] = L"Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDStorageFileHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDStorageFileHelper ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStorageFileHelper_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStorageFileHelper_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDStorageFileHelper[] = L"Windows.Media.Protection.PlayReady.NDStorageFileHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDStreamParserNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDStreamParserNotifier ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStreamParserNotifier_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStreamParserNotifier_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDStreamParserNotifier[] = L"Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDTCPMessenger
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDTCPMessengerFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDMessenger ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDTCPMessenger_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDTCPMessenger_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDTCPMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDTCPMessenger[] = L"Windows.Media.Protection.PlayReady.NDTCPMessenger";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyContentHeader ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentHeader_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentHeader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyContentHeader[] = L"Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyContentResolver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyContentResolver interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentResolver_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentResolver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyContentResolver[] = L"Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyDomain ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomain_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomain_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomain[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomain";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadyDomain> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterable_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainIterable[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadyDomain> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainIterator[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainJoinServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainJoinServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainJoinServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainLeaveServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainLeaveServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainLeaveServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyITADataGenerator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyITADataGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyITADataGenerator[] = L"Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyIndividualizationServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyIndividualizationServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyIndividualizationServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicense ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicense2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicense_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicense_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicense[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicense";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseAcquisitionServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseAcquisitionServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseAcquisitionServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadyLicense> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterable_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterable[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadyLicense> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterator[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseManagement_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseManagement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseManagement[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseSession_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseSession[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyMeteringReportServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyMeteringReportServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyMeteringReportServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyRevocationServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyRevocationServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyRevocationServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterable_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterable[] = L"Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterator[] = L"Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySecureStopServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySoapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadySoapMessage ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySoapMessage_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySoapMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySoapMessage[] = L"Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics5 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics4 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyStatics_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyStatics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyStatics[] = L"Windows.Media.Protection.PlayReady.PlayReadyStatics";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5 __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResultVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* This,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

typedef struct __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl;

interface __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

typedef struct __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        __FIIterator_1_Windows__CMedia__CCore__CAudioStreamDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl;

interface __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

typedef struct __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl;

interface __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

typedef struct __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        __FIIterator_1_Windows__CMedia__CCore__CVideoStreamDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl;

interface __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain;

typedef struct __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomainVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomainVtbl;

interface __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomainVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain;

typedef struct __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomainVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain* This,
        __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomainVtbl;

interface __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomainVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense;

typedef struct __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicenseVtbl;

interface __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense;

typedef struct __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense* This,
        __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicenseVtbl;

interface __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest;

typedef struct __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequestVtbl;

interface __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest;

typedef struct __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest* This,
        __FIIterator_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequestVtbl;

interface __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__
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
#if !defined(____FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

typedef struct __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl;

interface __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

typedef struct __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl;

interface __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__
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
#if !defined(____FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor;

typedef struct __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        __FIVectorView_1_Windows__CMedia__CCore__CAudioStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIAudioStreamDescriptor** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl;

interface __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor;

typedef struct __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        __FIVectorView_1_Windows__CMedia__CCore__CVideoStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIVideoStreamDescriptor** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl;

interface __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor_INTERFACE_DEFINED__
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
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* sender,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* sender,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* sender,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* sender,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CTimeSpan __x_ABI_CWindows_CFoundation_CTimeSpan;

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionServiceRequest_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager;

#endif // ____x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateFeature __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateFeature;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificatePlatformID __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificatePlatformID;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateType __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateType;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDClosedCaptionFormat __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDClosedCaptionFormat;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDContentIDType __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDContentIDType;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDMediaStreamType __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDMediaStreamType;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDProximityDetectionType __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDProximityDetectionType;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyDecryptorSetup __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyDecryptorSetup;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyEncryptionAlgorithm __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyEncryptionAlgorithm;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyHardwareDRMFeatures __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyHardwareDRMFeatures;

typedef enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyITADataFormat __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyITADataFormat;

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDCertificateFeature
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDCertificateFeature is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateFeature
{
    NDCertificateFeature_Transmitter = 1,
    NDCertificateFeature_Receiver = 2,
    NDCertificateFeature_SharedCertificate = 3,
    NDCertificateFeature_SecureClock = 4,
    NDCertificateFeature_AntiRollBackClock = 5,
    NDCertificateFeature_CRLS = 9,
    NDCertificateFeature_PlayReady3Features = 13,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDCertificatePlatformID
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDCertificatePlatformID is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificatePlatformID
{
    NDCertificatePlatformID_Windows = 0,
    NDCertificatePlatformID_OSX = 1,
    NDCertificatePlatformID_WindowsOnARM = 2,
    NDCertificatePlatformID_WindowsMobile7 = 5,
    NDCertificatePlatformID_iOSOnARM = 6,
    NDCertificatePlatformID_XBoxOnPPC = 7,
    NDCertificatePlatformID_WindowsPhone8OnARM = 8,
    NDCertificatePlatformID_WindowsPhone8OnX86 = 9,
    NDCertificatePlatformID_XboxOne = 10,
    NDCertificatePlatformID_AndroidOnARM = 11,
    NDCertificatePlatformID_WindowsPhone81OnARM = 12,
    NDCertificatePlatformID_WindowsPhone81OnX86 = 13,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDCertificateType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDCertificateType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateType
{
    NDCertificateType_Unknown = 0,
    NDCertificateType_PC = 1,
    NDCertificateType_Device = 2,
    NDCertificateType_Domain = 3,
    NDCertificateType_Issuer = 4,
    NDCertificateType_CrlSigner = 5,
    NDCertificateType_Service = 6,
    NDCertificateType_Silverlight = 7,
    NDCertificateType_Application = 8,
    NDCertificateType_Metering = 9,
    NDCertificateType_KeyFileSigner = 10,
    NDCertificateType_Server = 11,
    NDCertificateType_LicenseSigner = 12,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDClosedCaptionFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDClosedCaptionFormat is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDClosedCaptionFormat
{
    NDClosedCaptionFormat_ATSC = 0,
    NDClosedCaptionFormat_SCTE20 = 1,
    NDClosedCaptionFormat_Unknown = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDContentIDType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDContentIDType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDContentIDType
{
    NDContentIDType_KeyID = 1,
    NDContentIDType_PlayReadyObject = 2,
    NDContentIDType_Custom = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDMediaStreamType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDMediaStreamType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDMediaStreamType
{
    NDMediaStreamType_Audio = 1,
    NDMediaStreamType_Video = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDProximityDetectionType
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDProximityDetectionType is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDProximityDetectionType
{
    NDProximityDetectionType_UDP = 1,
    NDProximityDetectionType_TCP = 2,
    NDProximityDetectionType_TransportAgnostic = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.NDStartAsyncOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDStartAsyncOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDStartAsyncOptions
{
    NDStartAsyncOptions_MutualAuthentication = 1,
    NDStartAsyncOptions_WaitForLicenseDescriptor = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyDecryptorSetup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyDecryptorSetup
{
    PlayReadyDecryptorSetup_Uninitialized = 0,
    PlayReadyDecryptorSetup_OnDemand = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyEncryptionAlgorithm
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyEncryptionAlgorithm
{
    PlayReadyEncryptionAlgorithm_Unprotected = 0,
    PlayReadyEncryptionAlgorithm_Aes128Ctr = 1,
    PlayReadyEncryptionAlgorithm_Cocktail = 4,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    PlayReadyEncryptionAlgorithm_Aes128Cbc = 5,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    PlayReadyEncryptionAlgorithm_Unspecified = 65535,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    PlayReadyEncryptionAlgorithm_Uninitialized = 2147483647,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyHardwareDRMFeatures
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyHardwareDRMFeatures
{
    PlayReadyHardwareDRMFeatures_HardwareDRM = 1,
    PlayReadyHardwareDRMFeatures_HEVC = 2,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
    PlayReadyHardwareDRMFeatures_Aes128Cbc = 3,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.Protection.PlayReady.PlayReadyITADataFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyITADataFormat
{
    PlayReadyITADataFormat_SerializedProperties = 0,
    PlayReadyITADataFormat_SerializedProperties_WithContentProtectionWrapper = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDClient[] = L"Windows.Media.Protection.PlayReady.INDClient";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* add_RegistrationCompleted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDRegistrationCompletedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* remove_RegistrationCompleted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* add_ProximityDetectionCompleted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDProximityDetectionCompletedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* remove_ProximityDetectionCompleted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* add_LicenseFetchCompleted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchCompletedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* remove_LicenseFetchCompleted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* add_ReRegistrationNeeded)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_IInspectable* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* remove_ReRegistrationNeeded)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* add_ClosedCaptionDataReceived)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __FITypedEventHandler_2_Windows__CMedia__CProtection__CPlayReady__CNDClient_Windows__CMedia__CProtection__CPlayReady__CINDClosedCaptionDataReceivedEventArgs* handler,
        EventRegistrationToken* token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* remove_ClosedCaptionDataReceived)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        EventRegistrationToken token);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* StartAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* contentUrl,
        UINT32 startAsyncOptions,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* registrationCustomData,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* licenseFetchDescriptor,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDStartResult** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* LicenseFetchAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* licenseFetchDescriptor,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDLicenseFetchResult** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* ReRegistrationAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* registrationCustomData,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_add_RegistrationCompleted(This, handler, token) \
    ((This)->lpVtbl->add_RegistrationCompleted(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_remove_RegistrationCompleted(This, token) \
    ((This)->lpVtbl->remove_RegistrationCompleted(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_add_ProximityDetectionCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ProximityDetectionCompleted(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_remove_ProximityDetectionCompleted(This, token) \
    ((This)->lpVtbl->remove_ProximityDetectionCompleted(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_add_LicenseFetchCompleted(This, handler, token) \
    ((This)->lpVtbl->add_LicenseFetchCompleted(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_remove_LicenseFetchCompleted(This, token) \
    ((This)->lpVtbl->remove_LicenseFetchCompleted(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_add_ReRegistrationNeeded(This, handler, token) \
    ((This)->lpVtbl->add_ReRegistrationNeeded(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_remove_ReRegistrationNeeded(This, token) \
    ((This)->lpVtbl->remove_ReRegistrationNeeded(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_add_ClosedCaptionDataReceived(This, handler, token) \
    ((This)->lpVtbl->add_ClosedCaptionDataReceived(This, handler, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_remove_ClosedCaptionDataReceived(This, token) \
    ((This)->lpVtbl->remove_ClosedCaptionDataReceived(This, token))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_StartAsync(This, contentUrl, startAsyncOptions, registrationCustomData, licenseFetchDescriptor, result) \
    ((This)->lpVtbl->StartAsync(This, contentUrl, startAsyncOptions, registrationCustomData, licenseFetchDescriptor, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_LicenseFetchAsync(This, licenseFetchDescriptor, result) \
    ((This)->lpVtbl->LicenseFetchAsync(This, licenseFetchDescriptor, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_ReRegistrationAsync(This, registrationCustomData, result) \
    ((This)->lpVtbl->ReRegistrationAsync(This, registrationCustomData, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_Close(This) \
    ((This)->lpVtbl->Close(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDClientFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDClient
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDClientFactory[] = L"Windows.Media.Protection.PlayReady.INDClientFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDClientFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClientFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* downloadEngine,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* streamParser,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* pMessenger,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClient** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClientFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_CreateInstance(This, downloadEngine, streamParser, pMessenger, instance) \
    ((This)->lpVtbl->CreateInstance(This, downloadEngine, streamParser, pMessenger, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClientFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDClosedCaptionDataReceivedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDClosedCaptionDataReceivedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ClosedCaptionDataFormat)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDClosedCaptionFormat* ccForamt);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_PresentationTimestamp)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        INT64* presentationTimestamp);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ClosedCaptionData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs* This,
        UINT32* ccDataBytesLength,
        BYTE** ccDataBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_get_ClosedCaptionDataFormat(This, ccForamt) \
    ((This)->lpVtbl->get_ClosedCaptionDataFormat(This, ccForamt))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_get_PresentationTimestamp(This, presentationTimestamp) \
    ((This)->lpVtbl->get_PresentationTimestamp(This, presentationTimestamp))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDClosedCaptionDataReceivedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_get_ClosedCaptionData(This, ccDataBytesLength, ccDataBytes) \
    ((This)->lpVtbl->get_ClosedCaptionData(This, ccDataBytesLength, ccDataBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDClosedCaptionDataReceivedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDCustomData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDCustomData[] = L"Windows.Media.Protection.PlayReady.INDCustomData";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_CustomDataTypeID)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This,
        UINT32* customDataTypeIDBytesLength,
        BYTE** customDataTypeIDBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_CustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* This,
        UINT32* customDataBytesLength,
        BYTE** customDataBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_get_CustomDataTypeID(This, customDataTypeIDBytesLength, customDataTypeIDBytes) \
    ((This)->lpVtbl->get_CustomDataTypeID(This, customDataTypeIDBytesLength, customDataTypeIDBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_get_CustomData(This, customDataBytesLength, customDataBytes) \
    ((This)->lpVtbl->get_CustomData(This, customDataBytesLength, customDataBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDCustomDataFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDCustomData
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDCustomDataFactory[] = L"Windows.Media.Protection.PlayReady.INDCustomDataFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDCustomDataFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDCustomDataFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory* This,
        UINT32 customDataTypeIDBytesLength,
        BYTE* customDataTypeIDBytes,
        UINT32 customDataBytesLength,
        BYTE* customDataBytes,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDCustomDataFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_CreateInstance(This, customDataTypeIDBytesLength, customDataTypeIDBytes, customDataBytesLength, customDataBytes, instance) \
    ((This)->lpVtbl->CreateInstance(This, customDataTypeIDBytesLength, customDataTypeIDBytes, customDataBytesLength, customDataBytes, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomDataFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDDownloadEngine
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDDownloadEngine[] = L"Windows.Media.Protection.PlayReady.INDDownloadEngine";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* Open)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* uri,
        UINT32 sessionIDBytesLength,
        BYTE* sessionIDBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* Pause)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* Resume)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* Seek)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        struct __x_ABI_CWindows_CFoundation_CTimeSpan startPosition);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_CanSeek)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        boolean* canSeek);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_BufferFullMinThresholdInSamples)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        UINT32* bufferFullMinThreshold);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_BufferFullMaxThresholdInSamples)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        UINT32* bufferFullMaxThreshold);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_Notifier)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_Open(This, uri, sessionIDBytesLength, sessionIDBytes) \
    ((This)->lpVtbl->Open(This, uri, sessionIDBytesLength, sessionIDBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_Pause(This) \
    ((This)->lpVtbl->Pause(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_Resume(This) \
    ((This)->lpVtbl->Resume(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_Close(This) \
    ((This)->lpVtbl->Close(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_Seek(This, startPosition) \
    ((This)->lpVtbl->Seek(This, startPosition))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_get_CanSeek(This, canSeek) \
    ((This)->lpVtbl->get_CanSeek(This, canSeek))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_get_BufferFullMinThresholdInSamples(This, bufferFullMinThreshold) \
    ((This)->lpVtbl->get_BufferFullMinThresholdInSamples(This, bufferFullMinThreshold))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_get_BufferFullMaxThresholdInSamples(This, bufferFullMaxThreshold) \
    ((This)->lpVtbl->get_BufferFullMaxThresholdInSamples(This, bufferFullMaxThreshold))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngine is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_get_Notifier(This, instance) \
    ((This)->lpVtbl->get_Notifier(This, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngine_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDDownloadEngineNotifier[] = L"Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnStreamOpened)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnPlayReadyObjectReceived)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        UINT32 dataBytesLength,
        BYTE* dataBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnContentIDReceived)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* licenseFetchDescriptor);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnDataReceived)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This,
        UINT32 dataBytesLength,
        BYTE* dataBytes,
        UINT32 bytesReceived);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnEndOfStream)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnNetworkError)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifierVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_OnStreamOpened(This) \
    ((This)->lpVtbl->OnStreamOpened(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_OnPlayReadyObjectReceived(This, dataBytesLength, dataBytes) \
    ((This)->lpVtbl->OnPlayReadyObjectReceived(This, dataBytesLength, dataBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_OnContentIDReceived(This, licenseFetchDescriptor) \
    ((This)->lpVtbl->OnContentIDReceived(This, licenseFetchDescriptor))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_OnDataReceived(This, dataBytesLength, dataBytes, bytesReceived) \
    ((This)->lpVtbl->OnDataReceived(This, dataBytesLength, dataBytes, bytesReceived))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_OnEndOfStream(This) \
    ((This)->lpVtbl->OnEndOfStream(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_OnNetworkError(This) \
    ((This)->lpVtbl->OnNetworkError(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDDownloadEngineNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchCompletedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchCompletedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDLicenseFetchCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ResponseCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData** customData);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_get_ResponseCustomData(This, customData) \
    ((This)->lpVtbl->get_ResponseCustomData(This, customData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchDescriptor[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ContentIDType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDContentIDType* contentIDType);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ContentID)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        UINT32* contentIDBytesLength,
        BYTE** contentIDBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_LicenseFetchChallengeCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData** licenseFetchChallengeCustomData);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* put_LicenseFetchChallengeCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* licenseFetchChallengeCustomData);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_get_ContentIDType(This, contentIDType) \
    ((This)->lpVtbl->get_ContentIDType(This, contentIDType))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_get_ContentID(This, contentIDBytesLength, contentIDBytes) \
    ((This)->lpVtbl->get_ContentID(This, contentIDBytesLength, contentIDBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_get_LicenseFetchChallengeCustomData(This, licenseFetchChallengeCustomData) \
    ((This)->lpVtbl->get_LicenseFetchChallengeCustomData(This, licenseFetchChallengeCustomData))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_put_LicenseFetchChallengeCustomData(This, licenseFetchChallengeCustomData) \
    ((This)->lpVtbl->put_LicenseFetchChallengeCustomData(This, licenseFetchChallengeCustomData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchDescriptorFactory[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDLicenseFetchDescriptorFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptorFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDContentIDType contentIDType,
        UINT32 contentIDBytesLength,
        BYTE* contentIDBytes,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData* licenseFetchChallengeCustomData,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchDescriptorFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_CreateInstance(This, contentIDType, contentIDBytesLength, contentIDBytes, licenseFetchChallengeCustomData, instance) \
    ((This)->lpVtbl->CreateInstance(This, contentIDType, contentIDBytesLength, contentIDBytes, licenseFetchChallengeCustomData, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptorFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDLicenseFetchResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDLicenseFetchResult[] = L"Windows.Media.Protection.PlayReady.INDLicenseFetchResult";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDLicenseFetchResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ResponseCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData** customData);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResultVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDLicenseFetchResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_get_ResponseCustomData(This, customData) \
    ((This)->lpVtbl->get_ResponseCustomData(This, customData))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDMessenger
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDMessenger[] = L"Windows.Media.Protection.PlayReady.INDMessenger";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessengerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* SendRegistrationRequestAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        UINT32 sessionIDBytesLength,
        BYTE* sessionIDBytes,
        UINT32 challengeDataBytesLength,
        BYTE* challengeDataBytes,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* SendProximityDetectionStartAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDProximityDetectionType pdType,
        UINT32 transmitterChannelBytesLength,
        BYTE* transmitterChannelBytes,
        UINT32 sessionIDBytesLength,
        BYTE* sessionIDBytes,
        UINT32 challengeDataBytesLength,
        BYTE* challengeDataBytes,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* SendProximityDetectionResponseAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDProximityDetectionType pdType,
        UINT32 transmitterChannelBytesLength,
        BYTE* transmitterChannelBytes,
        UINT32 sessionIDBytesLength,
        BYTE* sessionIDBytes,
        UINT32 responseDataBytesLength,
        BYTE* responseDataBytes,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* SendLicenseFetchRequestAsync)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger* This,
        UINT32 sessionIDBytesLength,
        BYTE* sessionIDBytes,
        UINT32 challengeDataBytesLength,
        BYTE* challengeDataBytes,
        __FIAsyncOperation_1_Windows__CMedia__CProtection__CPlayReady__CINDSendResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessengerVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessengerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_SendRegistrationRequestAsync(This, sessionIDBytesLength, sessionIDBytes, challengeDataBytesLength, challengeDataBytes, result) \
    ((This)->lpVtbl->SendRegistrationRequestAsync(This, sessionIDBytesLength, sessionIDBytes, challengeDataBytesLength, challengeDataBytes, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_SendProximityDetectionStartAsync(This, pdType, transmitterChannelBytesLength, transmitterChannelBytes, sessionIDBytesLength, sessionIDBytes, challengeDataBytesLength, challengeDataBytes, result) \
    ((This)->lpVtbl->SendProximityDetectionStartAsync(This, pdType, transmitterChannelBytesLength, transmitterChannelBytes, sessionIDBytesLength, sessionIDBytes, challengeDataBytesLength, challengeDataBytes, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_SendProximityDetectionResponseAsync(This, pdType, transmitterChannelBytesLength, transmitterChannelBytes, sessionIDBytesLength, sessionIDBytes, responseDataBytesLength, responseDataBytes, result) \
    ((This)->lpVtbl->SendProximityDetectionResponseAsync(This, pdType, transmitterChannelBytesLength, transmitterChannelBytes, sessionIDBytesLength, sessionIDBytes, responseDataBytesLength, responseDataBytes, result))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_SendLicenseFetchRequestAsync(This, sessionIDBytesLength, sessionIDBytes, challengeDataBytesLength, challengeDataBytes, result) \
    ((This)->lpVtbl->SendLicenseFetchRequestAsync(This, sessionIDBytesLength, sessionIDBytes, challengeDataBytesLength, challengeDataBytes, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDProximityDetectionCompletedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDProximityDetectionCompletedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDProximityDetectionCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDProximityDetectionCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ProximityDetectionRetryCount)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs* This,
        UINT32* retryCount);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDProximityDetectionCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_get_ProximityDetectionRetryCount(This, retryCount) \
    ((This)->lpVtbl->get_ProximityDetectionRetryCount(This, retryCount))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDProximityDetectionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDRegistrationCompletedEventArgs[] = L"Windows.Media.Protection.PlayReady.INDRegistrationCompletedEventArgs";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ResponseCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDCustomData** customData);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_TransmitterProperties)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties** transmitterProperties);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_TransmitterCertificateAccepted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        boolean* acceptpt);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* put_TransmitterCertificateAccepted)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs* This,
        boolean accept);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_get_ResponseCustomData(This, customData) \
    ((This)->lpVtbl->get_ResponseCustomData(This, customData))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_get_TransmitterProperties(This, transmitterProperties) \
    ((This)->lpVtbl->get_TransmitterProperties(This, transmitterProperties))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_get_TransmitterCertificateAccepted(This, acceptpt) \
    ((This)->lpVtbl->get_TransmitterCertificateAccepted(This, acceptpt))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDRegistrationCompletedEventArgs is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_put_TransmitterCertificateAccepted(This, accept) \
    ((This)->lpVtbl->put_TransmitterCertificateAccepted(This, accept))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDRegistrationCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDSendResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDSendResult[] = L"Windows.Media.Protection.PlayReady.INDSendResult";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDSendResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDSendResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_Response)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult* This,
        UINT32* responseDataBytesLength,
        BYTE** responseDataBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResultVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDSendResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_get_Response(This, responseDataBytesLength, responseDataBytes) \
    ((This)->lpVtbl->get_Response(This, responseDataBytesLength, responseDataBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDSendResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStartResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStartResult[] = L"Windows.Media.Protection.PlayReady.INDStartResult";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDStartResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStartResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_MediaStreamSource)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSource** mediaStreamSource);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResultVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStartResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_get_MediaStreamSource(This, mediaStreamSource) \
    ((This)->lpVtbl->get_MediaStreamSource(This, mediaStreamSource))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStartResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStorageFileHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStorageFileHelper[] = L"Windows.Media.Protection.PlayReady.INDStorageFileHelper";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelperVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* GetFileURLs)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIVector_1_HSTRING** fileURLs);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelperVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelperVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_GetFileURLs(This, file, fileURLs) \
    ((This)->lpVtbl->GetFileURLs(This, file, fileURLs))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStorageFileHelper_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStreamParser
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStreamParser[] = L"Windows.Media.Protection.PlayReady.INDStreamParser";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* ParseData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        UINT32 dataBytesLength,
        BYTE* dataBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* GetStreamInformation)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* descriptor,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDMediaStreamType* streamType,
        UINT32* streamID);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* BeginOfStream)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* EndOfStream)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_Notifier)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_ParseData(This, dataBytesLength, dataBytes) \
    ((This)->lpVtbl->ParseData(This, dataBytesLength, dataBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_GetStreamInformation(This, descriptor, streamType, streamID) \
    ((This)->lpVtbl->GetStreamInformation(This, descriptor, streamType, streamID))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_BeginOfStream(This) \
    ((This)->lpVtbl->BeginOfStream(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_EndOfStream(This) \
    ((This)->lpVtbl->EndOfStream(This))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParser is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_get_Notifier(This, instance) \
    ((This)->lpVtbl->get_Notifier(This, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParser_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDStreamParserNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDStreamParserNotifier[] = L"Windows.Media.Protection.PlayReady.INDStreamParserNotifier";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifierVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnContentIDReceived)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDLicenseFetchDescriptor* licenseFetchDescriptor);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnMediaStreamDescriptorCreated)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor* audioStreamDescriptors,
        __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor* videoStreamDescriptors);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnSampleParsed)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        UINT32 streamID,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDMediaStreamType streamType,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamSample* streamSample,
        INT64 pts,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDClosedCaptionFormat ccFormat,
        UINT32 ccDataBytesLength,
        BYTE* ccDataBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* OnBeginSetupDecryptor)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* descriptor,
        GUID keyID,
        UINT32 proBytesLength,
        BYTE* proBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifierVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifierVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_OnContentIDReceived(This, licenseFetchDescriptor) \
    ((This)->lpVtbl->OnContentIDReceived(This, licenseFetchDescriptor))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_OnMediaStreamDescriptorCreated(This, audioStreamDescriptors, videoStreamDescriptors) \
    ((This)->lpVtbl->OnMediaStreamDescriptorCreated(This, audioStreamDescriptors, videoStreamDescriptors))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_OnSampleParsed(This, streamID, streamType, streamSample, pts, ccFormat, ccDataBytesLength, ccDataBytes) \
    ((This)->lpVtbl->OnSampleParsed(This, streamID, streamType, streamSample, pts, ccFormat, ccDataBytesLength, ccDataBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_OnBeginSetupDecryptor(This, descriptor, keyID, proBytesLength, proBytes) \
    ((This)->lpVtbl->OnBeginSetupDecryptor(This, descriptor, keyID, proBytesLength, proBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDStreamParserNotifier_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDTCPMessengerFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.NDTCPMessenger
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDTCPMessengerFactory[] = L"Windows.Media.Protection.PlayReady.INDTCPMessengerFactory";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDTCPMessengerFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTCPMessengerFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory* This,
        HSTRING remoteHostName,
        UINT32 remoteHostPort,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDMessenger** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTCPMessengerFactory is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_CreateInstance(This, remoteHostName, remoteHostPort, instance) \
    ((This)->lpVtbl->CreateInstance(This, remoteHostName, remoteHostPort, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTCPMessengerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.INDTransmitterProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_INDTransmitterProperties[] = L"Windows.Media.Protection.PlayReady.INDTransmitterProperties";
typedef struct
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        TrustLevel* trustLevel);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_CertificateType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateType* type);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_PlatformIdentifier)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificatePlatformID* identifier);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_SupportedFeatures)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        UINT32* featureSetsLength,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CNDCertificateFeature** featureSets);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_SecurityLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        UINT32* level);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_SecurityVersion)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        UINT32* securityVersion);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ExpirationDate)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* expirationDate);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ClientID)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        UINT32* clientIDBytesLength,
        BYTE** clientIDBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ModelDigest)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        UINT32* modelDigestBytesLength,
        BYTE** modelDigestBytes);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ModelManufacturerName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        HSTRING* modelManufacturerName);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ModelName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        HSTRING* modelName);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    HRESULT (STDMETHODCALLTYPE* get_ModelNumber)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties* This,
        HSTRING* modelNumber);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_CertificateType(This, type) \
    ((This)->lpVtbl->get_CertificateType(This, type))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_PlatformIdentifier(This, identifier) \
    ((This)->lpVtbl->get_PlatformIdentifier(This, identifier))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_SupportedFeatures(This, featureSetsLength, featureSets) \
    ((This)->lpVtbl->get_SupportedFeatures(This, featureSetsLength, featureSets))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_SecurityLevel(This, level) \
    ((This)->lpVtbl->get_SecurityLevel(This, level))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_SecurityVersion(This, securityVersion) \
    ((This)->lpVtbl->get_SecurityVersion(This, securityVersion))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_ExpirationDate(This, expirationDate) \
    ((This)->lpVtbl->get_ExpirationDate(This, expirationDate))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_ClientID(This, clientIDBytesLength, clientIDBytes) \
    ((This)->lpVtbl->get_ClientID(This, clientIDBytesLength, clientIDBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_ModelDigest(This, modelDigestBytesLength, modelDigestBytes) \
    ((This)->lpVtbl->get_ModelDigest(This, modelDigestBytesLength, modelDigestBytes))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_ModelManufacturerName(This, modelManufacturerName) \
    ((This)->lpVtbl->get_ModelManufacturerName(This, modelManufacturerName))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_ModelName(This, modelName) \
    ((This)->lpVtbl->get_ModelName(This, modelName))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
    DEPRECATED("INDTransmitterProperties is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_get_ModelNumber(This, modelNumber) \
    ((This)->lpVtbl->get_ModelNumber(This, modelNumber))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CINDTransmitterProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeader[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeader";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeyId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_KeyIdString)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_LicenseAcquisitionUrl)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_LicenseAcquisitionUserInterfaceUrl)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_EncryptionType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyEncryptionAlgorithm* value);
    HRESULT (STDMETHODCALLTYPE* get_CustomAttributes)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DecryptorSetup)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyDecryptorSetup* value);
    HRESULT (STDMETHODCALLTYPE* GetSerializedHeader)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        UINT32* headerBytesLength,
        BYTE** headerBytes);
    HRESULT (STDMETHODCALLTYPE* get_HeaderWithEmbeddedUpdates)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_KeyId(This, value) \
    ((This)->lpVtbl->get_KeyId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_KeyIdString(This, value) \
    ((This)->lpVtbl->get_KeyIdString(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_LicenseAcquisitionUrl(This, value) \
    ((This)->lpVtbl->get_LicenseAcquisitionUrl(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_LicenseAcquisitionUserInterfaceUrl(This, value) \
    ((This)->lpVtbl->get_LicenseAcquisitionUserInterfaceUrl(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_DomainServiceId(This, value) \
    ((This)->lpVtbl->get_DomainServiceId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_EncryptionType(This, value) \
    ((This)->lpVtbl->get_EncryptionType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_CustomAttributes(This, value) \
    ((This)->lpVtbl->get_CustomAttributes(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_DecryptorSetup(This, value) \
    ((This)->lpVtbl->get_DecryptorSetup(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_GetSerializedHeader(This, headerBytesLength, headerBytes) \
    ((This)->lpVtbl->GetSerializedHeader(This, headerBytesLength, headerBytes))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_get_HeaderWithEmbeddedUpdates(This, value) \
    ((This)->lpVtbl->get_HeaderWithEmbeddedUpdates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeader2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_KeyIds)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This,
        UINT32* contentKeyIdsLength,
        GUID** contentKeyIds);
    HRESULT (STDMETHODCALLTYPE* get_KeyIdStrings)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2* This,
        UINT32* contentKeyIdStringsLength,
        HSTRING** contentKeyIdStrings);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_get_KeyIds(This, contentKeyIdsLength, contentKeyIds) \
    ((This)->lpVtbl->get_KeyIds(This, contentKeyIdsLength, contentKeyIds))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_get_KeyIdStrings(This, contentKeyIdStringsLength, contentKeyIdStrings) \
    ((This)->lpVtbl->get_KeyIdStrings(This, contentKeyIdStringsLength, contentKeyIdStrings))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeaderFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceFromWindowsMediaDrmHeader)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        UINT32 headerBytesLength,
        BYTE* headerBytes,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* licenseAcquisitionUrl,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* licenseAcquisitionUserInterfaceUrl,
        HSTRING customAttributes,
        GUID domainServiceId,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader** instance);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceFromComponents)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        GUID contentKeyId,
        HSTRING contentKeyIdString,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyEncryptionAlgorithm contentEncryptionAlgorithm,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* licenseAcquisitionUrl,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* licenseAcquisitionUserInterfaceUrl,
        HSTRING customAttributes,
        GUID domainServiceId,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader** instance);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceFromPlayReadyHeader)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory* This,
        UINT32 headerBytesLength,
        BYTE* headerBytes,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_CreateInstanceFromWindowsMediaDrmHeader(This, headerBytesLength, headerBytes, licenseAcquisitionUrl, licenseAcquisitionUserInterfaceUrl, customAttributes, domainServiceId, instance) \
    ((This)->lpVtbl->CreateInstanceFromWindowsMediaDrmHeader(This, headerBytesLength, headerBytes, licenseAcquisitionUrl, licenseAcquisitionUserInterfaceUrl, customAttributes, domainServiceId, instance))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_CreateInstanceFromComponents(This, contentKeyId, contentKeyIdString, contentEncryptionAlgorithm, licenseAcquisitionUrl, licenseAcquisitionUserInterfaceUrl, customAttributes, domainServiceId, instance) \
    ((This)->lpVtbl->CreateInstanceFromComponents(This, contentKeyId, contentKeyIdString, contentEncryptionAlgorithm, licenseAcquisitionUrl, licenseAcquisitionUserInterfaceUrl, customAttributes, domainServiceId, instance))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_CreateInstanceFromPlayReadyHeader(This, headerBytesLength, headerBytes, instance) \
    ((This)->lpVtbl->CreateInstanceFromPlayReadyHeader(This, headerBytesLength, headerBytes, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentHeaderFactory2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceFromComponents2)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2* This,
        UINT32 dwFlags,
        UINT32 contentKeyIdsLength,
        GUID* contentKeyIds,
        UINT32 contentKeyIdStringsLength,
        HSTRING* contentKeyIdStrings,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyEncryptionAlgorithm contentEncryptionAlgorithm,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* licenseAcquisitionUrl,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* licenseAcquisitionUserInterfaceUrl,
        HSTRING customAttributes,
        GUID domainServiceId,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_CreateInstanceFromComponents2(This, dwFlags, contentKeyIdsLength, contentKeyIds, contentKeyIdStringsLength, contentKeyIdStrings, contentEncryptionAlgorithm, licenseAcquisitionUrl, licenseAcquisitionUserInterfaceUrl, customAttributes, domainServiceId, instance) \
    ((This)->lpVtbl->CreateInstanceFromComponents2(This, dwFlags, contentKeyIdsLength, contentKeyIds, contentKeyIdStringsLength, contentKeyIdStrings, contentEncryptionAlgorithm, licenseAcquisitionUrl, licenseAcquisitionUserInterfaceUrl, customAttributes, domainServiceId, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeaderFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyContentResolver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyContentResolver
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyContentResolver[] = L"Windows.Media.Protection.PlayReady.IPlayReadyContentResolver";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolverVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* ServiceRequest)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* contentHeader,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest** serviceRequest);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolverVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolverVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_ServiceRequest(This, contentHeader, serviceRequest) \
    ((This)->lpVtbl->ServiceRequest(This, contentHeader, serviceRequest))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentResolver_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomain[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomain";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_AccountId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_ServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Revision)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_FriendlyName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DomainJoinUrl)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_get_AccountId(This, value) \
    ((This)->lpVtbl->get_AccountId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_get_ServiceId(This, value) \
    ((This)->lpVtbl->get_ServiceId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_get_Revision(This, value) \
    ((This)->lpVtbl->get_Revision(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_get_FriendlyName(This, value) \
    ((This)->lpVtbl->get_FriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_get_DomainJoinUrl(This, value) \
    ((This)->lpVtbl->get_DomainJoinUrl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomain_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyDomainIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomainIterableFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory* This,
        GUID domainAccountId,
        __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyDomain** domainIterable);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_CreateInstance(This, domainAccountId, domainIterable) \
    ((This)->lpVtbl->CreateInstance(This, domainAccountId, domainIterable))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainIterableFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomainJoinServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DomainAccountId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_DomainAccountId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* get_DomainFriendlyName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_DomainFriendlyName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_get_DomainAccountId(This, value) \
    ((This)->lpVtbl->get_DomainAccountId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_put_DomainAccountId(This, value) \
    ((This)->lpVtbl->put_DomainAccountId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_get_DomainFriendlyName(This, value) \
    ((This)->lpVtbl->get_DomainFriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_put_DomainFriendlyName(This, value) \
    ((This)->lpVtbl->put_DomainFriendlyName(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_get_DomainServiceId(This, value) \
    ((This)->lpVtbl->get_DomainServiceId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_put_DomainServiceId(This, value) \
    ((This)->lpVtbl->put_DomainServiceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainJoinServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyDomainLeaveServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DomainAccountId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_DomainAccountId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        GUID value);
    HRESULT (STDMETHODCALLTYPE* get_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_get_DomainAccountId(This, value) \
    ((This)->lpVtbl->get_DomainAccountId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_put_DomainAccountId(This, value) \
    ((This)->lpVtbl->put_DomainAccountId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_get_DomainServiceId(This, value) \
    ((This)->lpVtbl->get_DomainServiceId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_put_DomainServiceId(This, value) \
    ((This)->lpVtbl->put_DomainServiceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyDomainLeaveServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyITADataGenerator[] = L"Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGeneratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GenerateData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator* This,
        GUID guidCPSystemId,
        UINT32 countOfStreams,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyITADataFormat format,
        UINT32* dataBytesLength,
        BYTE** dataBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGeneratorVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGeneratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_GenerateData(This, guidCPSystemId, countOfStreams, configuration, format, dataBytesLength, dataBytes) \
    ((This)->lpVtbl->GenerateData(This, guidCPSystemId, countOfStreams, configuration, format, dataBytesLength, dataBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyITADataGenerator_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyIndividualizationServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyIndividualizationServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicense[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicense";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FullyEvaluated)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_UsableForPlay)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpirationDate)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_ExpireAfterFirstPlay)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_DomainAccountID)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_ChainDepth)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* GetKIDAtChainDepth)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense* This,
        UINT32 chainDepth,
        GUID* kid);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_get_FullyEvaluated(This, value) \
    ((This)->lpVtbl->get_FullyEvaluated(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_get_UsableForPlay(This, value) \
    ((This)->lpVtbl->get_UsableForPlay(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_get_ExpirationDate(This, value) \
    ((This)->lpVtbl->get_ExpirationDate(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_get_ExpireAfterFirstPlay(This, value) \
    ((This)->lpVtbl->get_ExpireAfterFirstPlay(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_get_DomainAccountID(This, value) \
    ((This)->lpVtbl->get_DomainAccountID(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_get_ChainDepth(This, value) \
    ((This)->lpVtbl->get_ChainDepth(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_GetKIDAtChainDepth(This, chainDepth, kid) \
    ((This)->lpVtbl->GetKIDAtChainDepth(This, chainDepth, kid))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicense2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicense
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicense
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicense2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicense2";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SecureStopId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_SecurityLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_InMemoryOnly)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExpiresInRealTime)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_get_SecureStopId(This, value) \
    ((This)->lpVtbl->get_SecureStopId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_get_SecurityLevel(This, value) \
    ((This)->lpVtbl->get_SecurityLevel(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_get_InMemoryOnly(This, value) \
    ((This)->lpVtbl->get_InMemoryOnly(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_get_ExpiresInRealTime(This, value) \
    ((This)->lpVtbl->get_ExpiresInRealTime(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicense2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseAcquisitionServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ContentHeader)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader** value);
    HRESULT (STDMETHODCALLTYPE* put_ContentHeader)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* value);
    HRESULT (STDMETHODCALLTYPE* get_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* put_DomainServiceId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_get_ContentHeader(This, value) \
    ((This)->lpVtbl->get_ContentHeader(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_put_ContentHeader(This, value) \
    ((This)->lpVtbl->put_ContentHeader(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_get_DomainServiceId(This, value) \
    ((This)->lpVtbl->get_DomainServiceId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_put_DomainServiceId(This, value) \
    ((This)->lpVtbl->put_DomainServiceId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseAcquisitionServiceRequest2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_get_SessionId(This, value) \
    ((This)->lpVtbl->get_SessionId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseAcquisitionServiceRequest3[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateLicenseIterable)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* contentHeader,
        boolean fullyEvaluated,
        __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_CreateLicenseIterable(This, contentHeader, fullyEvaluated, result) \
    ((This)->lpVtbl->CreateLicenseIterable(This, contentHeader, fullyEvaluated, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseIterableFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* contentHeader,
        boolean fullyEvaluated,
        __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_CreateInstance(This, contentHeader, fullyEvaluated, instance) \
    ((This)->lpVtbl->CreateInstance(This, contentHeader, fullyEvaluated, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseIterableFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseManagement[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagementVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* DeleteLicenses)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* contentHeader,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagementVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagementVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_DeleteLicenses(This, contentHeader, operation) \
    ((This)->lpVtbl->DeleteLicenses(This, contentHeader, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseManagement_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseSession[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateLAServiceRequest)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseAcquisitionServiceRequest** serviceRequest);
    HRESULT (STDMETHODCALLTYPE* ConfigureMediaProtectionManager)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession* This,
        __x_ABI_CWindows_CMedia_CProtection_CIMediaProtectionManager* mpm);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_CreateLAServiceRequest(This, serviceRequest) \
    ((This)->lpVtbl->CreateLAServiceRequest(This, serviceRequest))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_ConfigureMediaProtectionManager(This, mpm) \
    ((This)->lpVtbl->ConfigureMediaProtectionManager(This, mpm))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseSession2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateLicenseIterable)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyContentHeader* contentHeader,
        boolean fullyEvaluated,
        __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadyLicense** licenseIterable);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_CreateLicenseIterable(This, contentHeader, fullyEvaluated, licenseIterable) \
    ((This)->lpVtbl->CreateLicenseIterable(This, contentHeader, fullyEvaluated, licenseIterable))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyLicenseSession
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyLicenseSessionFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet* configuration,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSession** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_CreateInstance(This, configuration, instance) \
    ((This)->lpVtbl->CreateInstance(This, configuration, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyLicenseSessionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyMeteringReportServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MeteringCertificate)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This,
        UINT32* meteringCertBytesLength,
        BYTE** meteringCertBytes);
    HRESULT (STDMETHODCALLTYPE* put_MeteringCertificate)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest* This,
        UINT32 meteringCertBytesLength,
        BYTE* meteringCertBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_get_MeteringCertificate(This, meteringCertBytesLength, meteringCertBytes) \
    ((This)->lpVtbl->get_MeteringCertificate(This, meteringCertBytesLength, meteringCertBytes))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_put_MeteringCertificate(This, meteringCertBytesLength, meteringCertBytes) \
    ((This)->lpVtbl->put_MeteringCertificate(This, meteringCertBytesLength, meteringCertBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyMeteringReportServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyRevocationServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyRevocationServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySecureStopIterableFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory* This,
        UINT32 publisherCertBytesLength,
        BYTE* publisherCertBytes,
        __FIIterable_1_Windows__CMedia__CProtection__CPlayReady__CIPlayReadySecureStopServiceRequest** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_CreateInstance(This, publisherCertBytesLength, publisherCertBytes, instance) \
    ((This)->lpVtbl->CreateInstance(This, publisherCertBytesLength, publisherCertBytes, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopIterableFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySecureStopServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SessionID)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_StartTime)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateTime)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Stopped)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PublisherCertificate)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest* This,
        UINT32* publisherCertBytesLength,
        BYTE** publisherCertBytes);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_get_SessionID(This, value) \
    ((This)->lpVtbl->get_SessionID(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_get_StartTime(This, value) \
    ((This)->lpVtbl->get_StartTime(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_get_UpdateTime(This, value) \
    ((This)->lpVtbl->get_UpdateTime(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_get_Stopped(This, value) \
    ((This)->lpVtbl->get_Stopped(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_get_PublisherCertificate(This, publisherCertBytesLength, publisherCertBytes) \
    ((This)->lpVtbl->get_PublisherCertificate(This, publisherCertBytesLength, publisherCertBytes))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySecureStopServiceRequestFactory[] = L"Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This,
        UINT32 publisherCertBytesLength,
        BYTE* publisherCertBytes,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest** instance);
    HRESULT (STDMETHODCALLTYPE* CreateInstanceFromSessionID)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory* This,
        GUID sessionID,
        UINT32 publisherCertBytesLength,
        BYTE* publisherCertBytes,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequest** instance);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactoryVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_CreateInstance(This, publisherCertBytesLength, publisherCertBytes, instance) \
    ((This)->lpVtbl->CreateInstance(This, publisherCertBytesLength, publisherCertBytes, instance))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_CreateInstanceFromSessionID(This, sessionID, publisherCertBytesLength, publisherCertBytes, instance) \
    ((This)->lpVtbl->CreateInstanceFromSessionID(This, sessionID, publisherCertBytesLength, publisherCertBytes, instance))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySecureStopServiceRequestFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyServiceRequest[] = L"Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequestVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_Uri)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ResponseCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ChallengeCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_ChallengeCustomData)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* BeginServiceRequest)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** action);
    HRESULT (STDMETHODCALLTYPE* NextServiceRequest)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest** serviceRequest);
    HRESULT (STDMETHODCALLTYPE* GenerateManualEnablingChallenge)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage** challengeMessage);
    HRESULT (STDMETHODCALLTYPE* ProcessManualEnablingResponse)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest* This,
        UINT32 responseBytesLength,
        BYTE* responseBytes,
        HRESULT* result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequestVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequestVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_put_Uri(This, value) \
    ((This)->lpVtbl->put_Uri(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_get_ResponseCustomData(This, value) \
    ((This)->lpVtbl->get_ResponseCustomData(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_get_ChallengeCustomData(This, value) \
    ((This)->lpVtbl->get_ChallengeCustomData(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_put_ChallengeCustomData(This, value) \
    ((This)->lpVtbl->put_ChallengeCustomData(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_BeginServiceRequest(This, action) \
    ((This)->lpVtbl->BeginServiceRequest(This, action))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_NextServiceRequest(This, serviceRequest) \
    ((This)->lpVtbl->NextServiceRequest(This, serviceRequest))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_GenerateManualEnablingChallenge(This, challengeMessage) \
    ((This)->lpVtbl->GenerateManualEnablingChallenge(This, challengeMessage))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_ProcessManualEnablingResponse(This, responseBytesLength, responseBytes, result) \
    ((This)->lpVtbl->ProcessManualEnablingResponse(This, responseBytesLength, responseBytes, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyServiceRequest_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadySoapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadySoapMessage
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadySoapMessage[] = L"Windows.Media.Protection.PlayReady.IPlayReadySoapMessage";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetMessageBody)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        UINT32* messageBodyBytesLength,
        BYTE** messageBodyBytes);
    HRESULT (STDMETHODCALLTYPE* get_MessageHeaders)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** messageUri);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessageVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_GetMessageBody(This, messageBodyBytesLength, messageBodyBytes) \
    ((This)->lpVtbl->GetMessageBody(This, messageBodyBytesLength, messageBodyBytes))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_get_MessageHeaders(This, value) \
    ((This)->lpVtbl->get_MessageHeaders(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_get_Uri(This, messageUri) \
    ((This)->lpVtbl->get_Uri(This, messageUri))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadySoapMessage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DomainJoinServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_DomainLeaveServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_IndividualizationServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_LicenseAcquirerServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_MeteringReportServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_RevocationServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_MediaProtectionSystemId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_PlayReadySecurityVersion)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_DomainJoinServiceRequestType(This, value) \
    ((This)->lpVtbl->get_DomainJoinServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_DomainLeaveServiceRequestType(This, value) \
    ((This)->lpVtbl->get_DomainLeaveServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_IndividualizationServiceRequestType(This, value) \
    ((This)->lpVtbl->get_IndividualizationServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_LicenseAcquirerServiceRequestType(This, value) \
    ((This)->lpVtbl->get_LicenseAcquirerServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_MeteringReportServiceRequestType(This, value) \
    ((This)->lpVtbl->get_MeteringReportServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_RevocationServiceRequestType(This, value) \
    ((This)->lpVtbl->get_RevocationServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_MediaProtectionSystemId(This, value) \
    ((This)->lpVtbl->get_MediaProtectionSystemId(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_get_PlayReadySecurityVersion(This, value) \
    ((This)->lpVtbl->get_PlayReadySecurityVersion(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics2[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics2";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PlayReadyCertificateSecurityLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_get_PlayReadyCertificateSecurityLevel(This, value) \
    ((This)->lpVtbl->get_PlayReadyCertificateSecurityLevel(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics3[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics3";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SecureStopServiceRequestType)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* CheckSupportedHardware)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3* This,
        enum __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CPlayReadyHardwareDRMFeatures hwdrmFeature,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_get_SecureStopServiceRequestType(This, value) \
    ((This)->lpVtbl->get_SecureStopServiceRequestType(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_CheckSupportedHardware(This, hwdrmFeature, value) \
    ((This)->lpVtbl->CheckSupportedHardware(This, hwdrmFeature, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics3
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics4[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics4";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InputTrustAuthorityToCreate)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ProtectionSystemId)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4* This,
        GUID* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_get_InputTrustAuthorityToCreate(This, value) \
    ((This)->lpVtbl->get_InputTrustAuthorityToCreate(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_get_ProtectionSystemId(This, value) \
    ((This)->lpVtbl->get_ProtectionSystemId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.Protection.PlayReady.IPlayReadyStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics4
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics3
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics2
 *     Windows.Media.Protection.PlayReady.IPlayReadyStatics
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_Protection_PlayReady_IPlayReadyStatics5[] = L"Windows.Media.Protection.PlayReady.IPlayReadyStatics5";
typedef struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_HardwareDRMDisabledAtTime)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_HardwareDRMDisabledUntilTime)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* ResetHardwareDRMDisabled)(__x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5* This);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5Vtbl;

interface __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_get_HardwareDRMDisabledAtTime(This, value) \
    ((This)->lpVtbl->get_HardwareDRMDisabledAtTime(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_get_HardwareDRMDisabledUntilTime(This, value) \
    ((This)->lpVtbl->get_HardwareDRMDisabledUntilTime(This, value))

#define __x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_ResetHardwareDRMDisabled(This) \
    ((This)->lpVtbl->ResetHardwareDRMDisabled(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CProtection_CPlayReady_CIPlayReadyStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDClient
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDClientFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDClient ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDClient_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDClient_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDClient is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDClient[] = L"Windows.Media.Protection.PlayReady.NDClient";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDCustomData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDCustomDataFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDCustomData ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDCustomData_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDCustomData_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDCustomData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDCustomData[] = L"Windows.Media.Protection.PlayReady.NDCustomData";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDDownloadEngineNotifier ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDDownloadEngineNotifier_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDDownloadEngineNotifier_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDDownloadEngineNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDDownloadEngineNotifier[] = L"Windows.Media.Protection.PlayReady.NDDownloadEngineNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptorFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDLicenseFetchDescriptor ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDLicenseFetchDescriptor_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDLicenseFetchDescriptor_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDLicenseFetchDescriptor is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDLicenseFetchDescriptor[] = L"Windows.Media.Protection.PlayReady.NDLicenseFetchDescriptor";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDStorageFileHelper
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDStorageFileHelper ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStorageFileHelper_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStorageFileHelper_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDStorageFileHelper is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDStorageFileHelper[] = L"Windows.Media.Protection.PlayReady.NDStorageFileHelper";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDStreamParserNotifier
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDStreamParserNotifier ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStreamParserNotifier_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDStreamParserNotifier_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDStreamParserNotifier is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDStreamParserNotifier[] = L"Windows.Media.Protection.PlayReady.NDStreamParserNotifier";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.NDTCPMessenger
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.INDTCPMessengerFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.INDMessenger ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDTCPMessenger_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_NDTCPMessenger_DEFINED
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
DEPRECATED("NDTCPMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_NDTCPMessenger[] = L"Windows.Media.Protection.PlayReady.NDTCPMessenger";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyContentHeader
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyContentHeaderFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyContentHeader ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyContentHeader2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentHeader_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentHeader_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyContentHeader[] = L"Windows.Media.Protection.PlayReady.PlayReadyContentHeader";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyContentResolver
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyContentResolver interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentResolver_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyContentResolver_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyContentResolver[] = L"Windows.Media.Protection.PlayReady.PlayReadyContentResolver";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomain
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyDomain ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomain_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomain_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomain[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomain";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyDomainIterableFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadyDomain> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterable_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainIterable[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainIterable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadyDomain> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainIterator[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyDomainJoinServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainJoinServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainJoinServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainJoinServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainJoinServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyDomainLeaveServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainLeaveServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyDomainLeaveServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyDomainLeaveServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyDomainLeaveServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyITADataGenerator ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyITADataGenerator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyITADataGenerator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyITADataGenerator[] = L"Windows.Media.Protection.PlayReady.PlayReadyITADataGenerator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyIndividualizationServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyIndividualizationServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyIndividualizationServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyIndividualizationServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyIndividualizationServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicense
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicense ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicense2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicense_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicense_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicense[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicense";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest2
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseAcquisitionServiceRequest3
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseAcquisitionServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseAcquisitionServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseAcquisitionServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseAcquisitionServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyLicenseIterableFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadyLicense> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterable_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterable[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseIterable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadyLicense> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseIterator[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyLicenseManagement interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseManagement_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseManagement_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseManagement[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseManagement";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyLicenseSession
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadyLicenseSessionFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyLicenseSession2
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseSession_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyLicenseSession_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyLicenseSession[] = L"Windows.Media.Protection.PlayReady.PlayReadyLicenseSession";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyMeteringReportServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyMeteringReportServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyMeteringReportServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyMeteringReportServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyMeteringReportServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadyRevocationServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyRevocationServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyRevocationServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyRevocationServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadyRevocationServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadySecureStopIterableFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterable`1<Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterable_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterable_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterable[] = L"Windows.Media.Protection.PlayReady.PlayReadySecureStopIterable";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IIterator`1<Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest> ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterator_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySecureStopIterator[] = L"Windows.Media.Protection.PlayReady.PlayReadySecureStopIterator";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequestFactory interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadySecureStopServiceRequest ** Default Interface **
 *    Windows.Media.Protection.PlayReady.IPlayReadyServiceRequest
 *    Windows.Media.Protection.IMediaProtectionServiceRequest
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopServiceRequest_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySecureStopServiceRequest_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySecureStopServiceRequest[] = L"Windows.Media.Protection.PlayReady.PlayReadySecureStopServiceRequest";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadySoapMessage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.Protection.PlayReady.IPlayReadySoapMessage ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySoapMessage_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadySoapMessage_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadySoapMessage[] = L"Windows.Media.Protection.PlayReady.PlayReadySoapMessage";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.Protection.PlayReady.PlayReadyStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics5 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics4 interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics3 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.Protection.PlayReady.IPlayReadyStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyStatics_DEFINED
#define RUNTIMECLASS_Windows_Media_Protection_PlayReady_PlayReadyStatics_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_Protection_PlayReady_PlayReadyStatics[] = L"Windows.Media.Protection.PlayReady.PlayReadyStatics";
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
#endif // __windows2Emedia2Eprotection2Eplayready_p_h__

#endif // __windows2Emedia2Eprotection2Eplayready_h__
