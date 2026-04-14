
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
#ifndef __windows2Emedia2Emediaproperties_h__
#define __windows2Emedia2Emediaproperties_h__
#ifndef __windows2Emedia2Emediaproperties_p_h__
#define __windows2Emedia2Emediaproperties_p_h__


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
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
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

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingProperties2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2 ABI::Windows::Media::MediaProperties::IAudioEncodingProperties2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingProperties3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3 ABI::Windows::Media::MediaProperties::IAudioEncodingProperties3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingPropertiesStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics ABI::Windows::Media::MediaProperties::IAudioEncodingPropertiesStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingPropertiesStatics2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2 ABI::Windows::Media::MediaProperties::IAudioEncodingPropertiesStatics2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAudioEncodingPropertiesWithFormatUserData;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData ABI::Windows::Media::MediaProperties::IAudioEncodingPropertiesWithFormatUserData

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IAv1ProfileIdsStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics ABI::Windows::Media::MediaProperties::IAv1ProfileIdsStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IContainerEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties ABI::Windows::Media::MediaProperties::IContainerEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IContainerEncodingProperties2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2 ABI::Windows::Media::MediaProperties::IContainerEncodingProperties2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IH264ProfileIdsStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics ABI::Windows::Media::MediaProperties::IH264ProfileIdsStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IHevcProfileIdsStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics ABI::Windows::Media::MediaProperties::IHevcProfileIdsStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IImageEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties ABI::Windows::Media::MediaProperties::IImageEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IImageEncodingProperties2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2 ABI::Windows::Media::MediaProperties::IImageEncodingProperties2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IImageEncodingPropertiesStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics ABI::Windows::Media::MediaProperties::IImageEncodingPropertiesStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IImageEncodingPropertiesStatics2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2 ABI::Windows::Media::MediaProperties::IImageEncodingPropertiesStatics2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IImageEncodingPropertiesStatics3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3 ABI::Windows::Media::MediaProperties::IImageEncodingPropertiesStatics3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfile2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2 ABI::Windows::Media::MediaProperties::IMediaEncodingProfile2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfile3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3 ABI::Windows::Media::MediaProperties::IMediaEncodingProfile3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfileStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics ABI::Windows::Media::MediaProperties::IMediaEncodingProfileStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfileStatics2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2 ABI::Windows::Media::MediaProperties::IMediaEncodingProfileStatics2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfileStatics3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3 ABI::Windows::Media::MediaProperties::IMediaEncodingProfileStatics3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingProfileStatics4;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4 ABI::Windows::Media::MediaProperties::IMediaEncodingProfileStatics4

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2 ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3 ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics4;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4 ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics4

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics5;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5 ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics5

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics6;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6 ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics6

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMediaEncodingSubtypesStatics7;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7 ABI::Windows::Media::MediaProperties::IMediaEncodingSubtypesStatics7

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_FWD_DEFINED__

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

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IMpeg2ProfileIdsStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics ABI::Windows::Media::MediaProperties::IMpeg2ProfileIdsStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface ITimedMetadataEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties ABI::Windows::Media::MediaProperties::ITimedMetadataEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface ITimedMetadataEncodingPropertiesStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics ABI::Windows::Media::MediaProperties::ITimedMetadataEncodingPropertiesStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties ABI::Windows::Media::MediaProperties::IVideoEncodingProperties

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2 ABI::Windows::Media::MediaProperties::IVideoEncodingProperties2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3 ABI::Windows::Media::MediaProperties::IVideoEncodingProperties3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties4;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4 ABI::Windows::Media::MediaProperties::IVideoEncodingProperties4

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingProperties5;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5 ABI::Windows::Media::MediaProperties::IVideoEncodingProperties5

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingPropertiesStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics ABI::Windows::Media::MediaProperties::IVideoEncodingPropertiesStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingPropertiesStatics2;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2 ABI::Windows::Media::MediaProperties::IVideoEncodingPropertiesStatics2

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVideoEncodingPropertiesStatics3;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3 ABI::Windows::Media::MediaProperties::IVideoEncodingPropertiesStatics3

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                interface IVp9ProfileIdsStatics;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics ABI::Windows::Media::MediaProperties::IVp9ProfileIdsStatics

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C++)

// Collection interface definitions
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaEncodingProfile;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_USE
#define DEF___FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d4f86f16-c6cf-57c8-9743-5ec20c31ab79"))
IAsyncOperation<ABI::Windows::Media::MediaProperties::MediaEncodingProfile*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::MediaEncodingProfile*, ABI::Windows::Media::MediaProperties::IMediaEncodingProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Media.MediaProperties.MediaEncodingProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Media::MediaProperties::MediaEncodingProfile*> __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_t;
#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("37296fc1-86da-58a0-90c0-c807bd94395e"))
IAsyncOperationCompletedHandler<ABI::Windows::Media::MediaProperties::MediaEncodingProfile*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::MediaProperties::MediaEncodingProfile*, ABI::Windows::Media::MediaProperties::IMediaEncodingProfile*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Media.MediaProperties.MediaEncodingProfile>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Media::MediaProperties::MediaEncodingProfile*> __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000


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
                class TimedMetadataStreamDescriptor;
            } /* Core */
        } /* Media */
    } /* Windows */
} /* ABI */

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#define DEF___FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("68475671-f53b-57ff-92ac-28bfd46573d7"))
IIterator<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*, ABI::Windows::Media::Core::IMediaStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Media.Core.TimedMetadataStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t;
#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#define DEF___FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("f3d07841-3852-509d-a12b-a9f2ac89da93"))
IIterable<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*, ABI::Windows::Media::Core::IMediaStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Media.Core.TimedMetadataStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t;
#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#define DEF___FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("62e7b4ed-0c95-5743-a142-054f4bdea0a4"))
IVectorView<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*, ABI::Windows::Media::Core::IMediaStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Media.Core.TimedMetadataStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t;
#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#define DEF___FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1ebdbbcf-4f75-5645-b8bc-31a716978bcc"))
IVector<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*, ABI::Windows::Media::Core::IMediaStreamDescriptor*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.Media.Core.TimedMetadataStreamDescriptor>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::Media::Core::TimedMetadataStreamDescriptor*> __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t;
#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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
        namespace Media {
            namespace MediaProperties {
                typedef enum AudioEncodingQuality : int AudioEncodingQuality;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

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
                typedef enum SphericalVideoFrameFormat : int SphericalVideoFrameFormat;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum StereoscopicVideoPackingMode : int StereoscopicVideoPackingMode;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                typedef enum VideoEncodingQuality : int VideoEncodingQuality;
            } /* MediaProperties */
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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class ContainerEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class ImageEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class MediaPropertySet;
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

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class TimedMetadataEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                class VideoEncodingProperties;
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Media.MediaProperties.AudioEncodingQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum AudioEncodingQuality : int
                {
                    AudioEncodingQuality_Auto = 0,
                    AudioEncodingQuality_High = 1,
                    AudioEncodingQuality_Medium = 2,
                    AudioEncodingQuality_Low = 3,
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaMirroringOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum MediaMirroringOptions : unsigned int
                {
                    MediaMirroringOptions_None = 0,
                    MediaMirroringOptions_Horizontal = 0x1,
                    MediaMirroringOptions_Vertical = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(MediaMirroringOptions)
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaPixelFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum MediaPixelFormat : int
                {
                    MediaPixelFormat_Nv12 = 0,
                    MediaPixelFormat_Bgra8 = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                    MediaPixelFormat_P010 = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum MediaRotation : int
                {
                    MediaRotation_None = 0,
                    MediaRotation_Clockwise90Degrees = 1,
                    MediaRotation_Clockwise180Degrees = 2,
                    MediaRotation_Clockwise270Degrees = 3,
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaThumbnailFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum MediaThumbnailFormat : int
                {
                    MediaThumbnailFormat_Bmp = 0,
                    MediaThumbnailFormat_Bgra8 = 1,
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.SphericalVideoFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum SphericalVideoFrameFormat : int
                {
                    SphericalVideoFrameFormat_None = 0,
                    SphericalVideoFrameFormat_Unsupported = 1,
                    SphericalVideoFrameFormat_Equirectangular = 2,
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.MediaProperties.StereoscopicVideoPackingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum StereoscopicVideoPackingMode : int
                {
                    StereoscopicVideoPackingMode_None = 0,
                    StereoscopicVideoPackingMode_SideBySide = 1,
                    StereoscopicVideoPackingMode_TopBottom = 2,
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.MediaProperties.VideoEncodingQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                enum VideoEncodingQuality : int
                {
                    VideoEncodingQuality_Auto = 0,
                    VideoEncodingQuality_HD1080p = 1,
                    VideoEncodingQuality_HD720p = 2,
                    VideoEncodingQuality_Wvga = 3,
                    VideoEncodingQuality_Ntsc = 4,
                    VideoEncodingQuality_Pal = 5,
                    VideoEncodingQuality_Vga = 6,
                    VideoEncodingQuality_Qvga = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    VideoEncodingQuality_Uhd2160p = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                    VideoEncodingQuality_Uhd4320p = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
                };
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingProperties[] = L"Windows.Media.MediaProperties.IAudioEncodingProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("62bc7a16-005c-4b3b-8a0b-0a090e9687f3")
                IAudioEncodingProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Bitrate(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bitrate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ChannelCount(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ChannelCount(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_SampleRate(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SampleRate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_BitsPerSample(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_BitsPerSample(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEncodingProperties = __uuidof(IAudioEncodingProperties);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingProperties2[] = L"Windows.Media.MediaProperties.IAudioEncodingProperties2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("c45d54da-80bd-4c23-80d5-72d4a181e894")
                IAudioEncodingProperties2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsSpatial(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEncodingProperties2 = __uuidof(IAudioEncodingProperties2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingProperties3[] = L"Windows.Media.MediaProperties.IAudioEncodingProperties3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("87600341-748c-4f8d-b0fd-10caf08ff087")
                IAudioEncodingProperties3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Copy(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEncodingProperties3 = __uuidof(IAudioEncodingProperties3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("0cad332c-ebe9-4527-b36d-e42a13cf38db")
                IAudioEncodingPropertiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAac(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitrate,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAacAdts(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitrate,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMp3(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitrate,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePcm(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitsPerSample,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWma(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitrate,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEncodingPropertiesStatics = __uuidof(IAudioEncodingPropertiesStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingPropertiesStatics2[] = L"Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("7489316f-77a0-433d-8ed5-4040280e8665")
                IAudioEncodingPropertiesStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAlac(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitsPerSample,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFlac(
                        UINT32 sampleRate,
                        UINT32 channelCount,
                        UINT32 bitsPerSample,
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEncodingPropertiesStatics2 = __uuidof(IAudioEncodingPropertiesStatics2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingPropertiesWithFormatUserData[] = L"Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("98f10d79-13ea-49ff-be70-2673db69702c")
                IAudioEncodingPropertiesWithFormatUserData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetFormatUserData(
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFormatUserData(
                        UINT32* valueLength,
                        BYTE** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAudioEncodingPropertiesWithFormatUserData = __uuidof(IAudioEncodingPropertiesWithFormatUserData);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAv1ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.Av1ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAv1ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IAv1ProfileIdsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("9105812b-7c09-5882-88a4-678008a5174d")
                IAv1ProfileIdsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling420BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling420BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling400BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling400BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HighChromaSubsampling444BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HighChromaSubsampling444BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfessionalChromaSubsampling420BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfessionalChromaSubsampling400BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfessionalChromaSubsampling444BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfessionalChromaSubsampling422BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfessionalChromaSubsampling422BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfessionalChromaSubsampling422BitDepth12(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IAv1ProfileIdsStatics = __uuidof(IAv1ProfileIdsStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IContainerEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ContainerEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IContainerEncodingProperties[] = L"Windows.Media.MediaProperties.IContainerEncodingProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("59ac2a57-b32a-479e-8a61-4b7f2e9e7ea0")
                IContainerEncodingProperties : public IInspectable
                {
                public:
                };

                MIDL_CONST_ID IID& IID_IContainerEncodingProperties = __uuidof(IContainerEncodingProperties);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IContainerEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ContainerEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IContainerEncodingProperties2[] = L"Windows.Media.MediaProperties.IContainerEncodingProperties2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("b272c029-ae26-4819-baad-ad7a49b0a876")
                IContainerEncodingProperties2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Copy(
                        ABI::Windows::Media::MediaProperties::IContainerEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IContainerEncodingProperties2 = __uuidof(IContainerEncodingProperties2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IH264ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.H264ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IH264ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IH264ProfileIdsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("38654ca7-846a-4f97-a2e5-c3a15bbf70fd")
                IH264ProfileIdsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ConstrainedBaseline(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Baseline(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Extended(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Main(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_High(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_High10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_High422(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_High444(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_StereoHigh(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MultiviewHigh(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IH264ProfileIdsStatics = __uuidof(IH264ProfileIdsStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IHevcProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.HevcProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IHevcProfileIdsStatics[] = L"Windows.Media.MediaProperties.IHevcProfileIdsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("1e50d280-2aa7-53c1-973f-2189fa656f53")
                IHevcProfileIdsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling420BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling420BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling420BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling422BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling422BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling444BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling444BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainChromaSubsampling444BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MonochromeBitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MonochromeBitDepth16(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling420BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling420BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling420BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling422BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling422BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling444BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling444BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling444BitDepth12(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainIntraChromaSubsampling444BitDepth16(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainStillChromaSubsampling420BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainStillChromaSubsampling444BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MainStillChromaSubsampling444BitDepth16(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHevcProfileIdsStatics = __uuidof(IHevcProfileIdsStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingProperties[] = L"Windows.Media.MediaProperties.IImageEncodingProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("78625635-f331-4189-b1c3-b48d5ae034f1")
                IImageEncodingProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Width(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Height(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageEncodingProperties = __uuidof(IImageEncodingProperties);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingProperties2[] = L"Windows.Media.MediaProperties.IImageEncodingProperties2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("c854a2df-c923-469b-ac8e-6a9f3c1cd9e3")
                IImageEncodingProperties2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Copy(
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageEncodingProperties2 = __uuidof(IImageEncodingProperties2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.IImageEncodingPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("257c68dc-8b99-439e-aa59-913a36161297")
                IImageEncodingPropertiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateJpeg(
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreatePng(
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateJpegXR(
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageEncodingPropertiesStatics = __uuidof(IImageEncodingPropertiesStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingPropertiesStatics2[] = L"Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("f6c25b29-3824-46b0-956e-501329e1be3c")
                IImageEncodingPropertiesStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateUncompressed(
                        ABI::Windows::Media::MediaProperties::MediaPixelFormat format,
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateBmp(
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageEncodingPropertiesStatics2 = __uuidof(IImageEncodingPropertiesStatics2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingPropertiesStatics3[] = L"Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("48f4814d-a2ff-48dc-8ea0-e90680663656")
                IImageEncodingPropertiesStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateHeif(
                        ABI::Windows::Media::MediaProperties::IImageEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IImageEncodingPropertiesStatics3 = __uuidof(IImageEncodingPropertiesStatics3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfile[] = L"Windows.Media.MediaProperties.IMediaEncodingProfile";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("e7dbf5a8-1db9-4783-876b-3dfe12acfdb3")
                IMediaEncodingProfile : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Audio(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Audio(
                        ABI::Windows::Media::MediaProperties::IAudioEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Video(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Video(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Container(
                        ABI::Windows::Media::MediaProperties::IContainerEncodingProperties* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Container(
                        ABI::Windows::Media::MediaProperties::IContainerEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfile = __uuidof(IMediaEncodingProfile);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfile2[] = L"Windows.Media.MediaProperties.IMediaEncodingProfile2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("349b3e0a-4035-488e-9877-85632865ed10")
                IMediaEncodingProfile2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetAudioTracks(
                        __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetAudioTracks(
                        __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetVideoTracks(
                        __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetVideoTracks(
                        __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfile2 = __uuidof(IMediaEncodingProfile2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfile3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfile3[] = L"Windows.Media.MediaProperties.IMediaEncodingProfile3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("ba6ebe88-7570-4e69-accf-5611ad015f88")
                IMediaEncodingProfile3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetTimedMetadataTracks(
                        __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetTimedMetadataTracks(
                        __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfile3 = __uuidof(IMediaEncodingProfile3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("197f352c-2ede-4a45-a896-817a4854f8fe")
                IMediaEncodingProfileStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateM4a(
                        ABI::Windows::Media::MediaProperties::AudioEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMp3(
                        ABI::Windows::Media::MediaProperties::AudioEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWma(
                        ABI::Windows::Media::MediaProperties::AudioEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMp4(
                        ABI::Windows::Media::MediaProperties::VideoEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateWmv(
                        ABI::Windows::Media::MediaProperties::VideoEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromFileAsync(
                        ABI::Windows::Storage::IStorageFile* file,
                        __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFromStreamAsync(
                        ABI::Windows::Storage::Streams::IRandomAccessStream* stream,
                        __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfileStatics = __uuidof(IMediaEncodingProfileStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics2[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("ce8de74f-6af4-4288-8fe2-79adf1f79a43")
                IMediaEncodingProfileStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateWav(
                        ABI::Windows::Media::MediaProperties::AudioEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAvi(
                        ABI::Windows::Media::MediaProperties::VideoEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfileStatics2 = __uuidof(IMediaEncodingProfileStatics2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics3[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("90dac5aa-cf76-4294-a9ed-1a1420f51f6b")
                IMediaEncodingProfileStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateAlac(
                        ABI::Windows::Media::MediaProperties::AudioEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateFlac(
                        ABI::Windows::Media::MediaProperties::AudioEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateHevc(
                        ABI::Windows::Media::MediaProperties::VideoEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfileStatics3 = __uuidof(IMediaEncodingProfileStatics3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics4[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics4";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("6fafd7b5-9404-514a-81dd-c9444d648af0")
                IMediaEncodingProfileStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVp9(
                        ABI::Windows::Media::MediaProperties::VideoEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAv1(
                        ABI::Windows::Media::MediaProperties::VideoEncodingQuality quality,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProfile** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProfileStatics4 = __uuidof(IMediaEncodingProfileStatics4);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProperties[] = L"Windows.Media.MediaProperties.IMediaEncodingProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("b4002af6-acd4-4e5a-a24b-5d7498a8b8c4")
                IMediaEncodingProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        __FIMap_2_GUID_IInspectable** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Subtype(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Subtype(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingProperties = __uuidof(IMediaEncodingProperties);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("37b6580e-a171-4464-ba5a-53189e48c1c8")
                IMediaEncodingSubtypesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Aac(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AacAdts(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ac3(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AmrNb(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AmrWb(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Argb32(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Asf(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Avi(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bgra8(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bmp(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Eac3(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Float(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Gif(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_H263(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_H264(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_H264Es(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Hevc(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_HevcEs(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Iyuv(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Jpeg(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_JpegXr(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mjpg(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mpeg(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mpeg1(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mpeg2(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mp3(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Mpeg4(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Nv12(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Pcm(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Png(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rgb24(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Rgb32(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Tiff(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wave(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wma8(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wma9(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wmv3(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Wvc1(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Yuy2(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Yv12(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics = __uuidof(IMediaEncodingSubtypesStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics2[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("4b7cd23d-42ff-4d33-8531-0626bee4b52d")
                IMediaEncodingSubtypesStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Vp9(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_L8(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_L16(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_D16(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics2 = __uuidof(IMediaEncodingSubtypesStatics2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics3[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("ba2414e4-883d-464e-a44f-097da08ef7ff")
                IMediaEncodingSubtypesStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Alac(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Flac(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics3 = __uuidof(IMediaEncodingSubtypesStatics3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics4[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("ddece58a-3949-4644-8a2c-59ef02c642fa")
                IMediaEncodingSubtypesStatics4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_P010(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics4 = __uuidof(IMediaEncodingSubtypesStatics4);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics5[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("5ad4a007-ffce-4760-9828-5d0c99637e6a")
                IMediaEncodingSubtypesStatics5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Heif(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics5 = __uuidof(IMediaEncodingSubtypesStatics5);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics6[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("a1252973-a984-5912-93bb-54e7e569e053")
                IMediaEncodingSubtypesStatics6 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Pgs(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Srt(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Ssa(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_VobSub(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics6 = __uuidof(IMediaEncodingSubtypesStatics6);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics7[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics7";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("92f2dca7-9937-52a1-b619-ddfad81cd99c")
                IMediaEncodingSubtypesStatics7 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Av1(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaEncodingSubtypesStatics7 = __uuidof(IMediaEncodingSubtypesStatics7);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaRatio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaRatio
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaRatio[] = L"Windows.Media.MediaProperties.IMediaRatio";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("d2d0fee5-8929-401d-ac78-7d357e378163")
                IMediaRatio : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Numerator(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Numerator(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Denominator(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Denominator(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMediaRatio = __uuidof(IMediaRatio);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.Mpeg2ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMpeg2ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("a461ff85-e57a-4128-9b21-d5331b04235c")
                IMpeg2ProfileIdsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Simple(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Main(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SignalNoiseRatioScalable(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SpatiallyScalable(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_High(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMpeg2ProfileIdsStatics = __uuidof(IMpeg2ProfileIdsStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.ITimedMetadataEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.TimedMetadataEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_ITimedMetadataEncodingProperties[] = L"Windows.Media.MediaProperties.ITimedMetadataEncodingProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("51cd30d3-d690-4cfa-97f4-4a398e9db420")
                ITimedMetadataEncodingProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetFormatUserData(
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFormatUserData(
                        UINT32* valueLength,
                        BYTE** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Copy(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITimedMetadataEncodingProperties = __uuidof(ITimedMetadataEncodingProperties);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.TimedMetadataEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_ITimedMetadataEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("6629bb67-6e55-5643-89a0-7a7e8d85b52c")
                ITimedMetadataEncodingPropertiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreatePgs(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSrt(
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateSsa(
                        UINT32 formatUserDataLength,
                        BYTE* formatUserData,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateVobSub(
                        UINT32 formatUserDataLength,
                        BYTE* formatUserData,
                        ABI::Windows::Media::MediaProperties::IMediaEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_ITimedMetadataEncodingPropertiesStatics = __uuidof(ITimedMetadataEncodingPropertiesStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("76ee6c9a-37c2-4f2a-880a-1282bbb4373d")
                IVideoEncodingProperties : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE put_Bitrate(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Bitrate(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Width(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Width(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Height(
                        UINT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Height(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FrameRate(
                        ABI::Windows::Media::MediaProperties::IMediaRatio** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PixelAspectRatio(
                        ABI::Windows::Media::MediaProperties::IMediaRatio** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingProperties = __uuidof(IVideoEncodingProperties);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties2[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("f743a1ef-d465-4290-a94b-ef0f1528f8e3")
                IVideoEncodingProperties2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE SetFormatUserData(
                        UINT32 valueLength,
                        BYTE* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetFormatUserData(
                        UINT32* valueLength,
                        BYTE** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ProfileId(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProfileId(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingProperties2 = __uuidof(IVideoEncodingProperties2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties3[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("386bcdc4-873a-479f-b3eb-56c1fcbec6d7")
                IVideoEncodingProperties3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_StereoscopicVideoPackingMode(
                        ABI::Windows::Media::MediaProperties::StereoscopicVideoPackingMode* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingProperties3 = __uuidof(IVideoEncodingProperties3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties4[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties4";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("724ef014-c10c-40f2-9d72-3ee13b45fa8e")
                IVideoEncodingProperties4 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_SphericalVideoFrameFormat(
                        ABI::Windows::Media::MediaProperties::SphericalVideoFrameFormat* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingProperties4 = __uuidof(IVideoEncodingProperties4);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties5[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties5";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("4959080f-272f-4ece-a4df-c0ccdb33d840")
                IVideoEncodingProperties5 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Copy(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingProperties5 = __uuidof(IVideoEncodingProperties5);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("3ce14d44-1dc5-43db-9f38-ebebf90152cb")
                IVideoEncodingPropertiesStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateH264(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateMpeg2(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateUncompressed(
                        HSTRING subtype,
                        UINT32 width,
                        UINT32 height,
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingPropertiesStatics = __uuidof(IVideoEncodingPropertiesStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingPropertiesStatics2[] = L"Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("cf1ebd5d-49fe-4d00-b59a-cfa4dfc51944")
                IVideoEncodingPropertiesStatics2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateHevc(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingPropertiesStatics2 = __uuidof(IVideoEncodingPropertiesStatics2);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingPropertiesStatics3[] = L"Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics3";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("65b46685-60da-5e51-91a2-b38c4763b872")
                IVideoEncodingPropertiesStatics3 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateVp9(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateAv1(
                        ABI::Windows::Media::MediaProperties::IVideoEncodingProperties** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVideoEncodingPropertiesStatics3 = __uuidof(IVideoEncodingPropertiesStatics3);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IVp9ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.Vp9ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVp9ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IVp9ProfileIdsStatics";
namespace ABI {
    namespace Windows {
        namespace Media {
            namespace MediaProperties {
                MIDL_INTERFACE("20311a55-fe06-5883-92d9-6080c97743e5")
                IVp9ProfileIdsStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Profile0ChromaSubsampling420BitDepth8(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Profile2ChromaSubsampling420BitDepth10(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Profile2ChromaSubsampling420BitDepth12(
                        INT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IVp9ProfileIdsStatics = __uuidof(IVp9ProfileIdsStatics);
            } /* MediaProperties */
        } /* Media */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Media.MediaProperties.AudioEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IAudioEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData
 *    Windows.Media.MediaProperties.IAudioEncodingProperties2
 *    Windows.Media.MediaProperties.IAudioEncodingProperties3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_AudioEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_AudioEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_AudioEncodingProperties[] = L"Windows.Media.MediaProperties.AudioEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.Av1ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IAv1ProfileIdsStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_Av1ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_Av1ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_Av1ProfileIds[] = L"Windows.Media.MediaProperties.Av1ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Media.MediaProperties.ContainerEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IContainerEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IContainerEncodingProperties2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_ContainerEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_ContainerEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_ContainerEncodingProperties[] = L"Windows.Media.MediaProperties.ContainerEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.H264ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IH264ProfileIdsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_H264ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_H264ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_H264ProfileIds[] = L"Windows.Media.MediaProperties.H264ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.HevcProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IHevcProfileIdsStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_HevcProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_HevcProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_HevcProfileIds[] = L"Windows.Media.MediaProperties.HevcProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Media.MediaProperties.ImageEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IImageEncodingPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IImageEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IImageEncodingProperties2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_ImageEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_ImageEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_ImageEncodingProperties[] = L"Windows.Media.MediaProperties.ImageEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaEncodingProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics4 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IMediaEncodingProfile ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProfile2
 *    Windows.Media.MediaProperties.IMediaEncodingProfile3
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingProfile_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaEncodingProfile[] = L"Windows.Media.MediaProperties.MediaEncodingProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics7 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingSubtypes_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingSubtypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaEncodingSubtypes[] = L"Windows.Media.MediaProperties.MediaEncodingSubtypes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaPropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<Guid, Object> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<Guid, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaPropertySet_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaPropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaPropertySet[] = L"Windows.Media.MediaProperties.MediaPropertySet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaRatio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IMediaRatio ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaRatio_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaRatio_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaRatio[] = L"Windows.Media.MediaProperties.MediaRatio";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.Mpeg2ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_Mpeg2ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_Mpeg2ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_Mpeg2ProfileIds[] = L"Windows.Media.MediaProperties.Mpeg2ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.TimedMetadataEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.ITimedMetadataEncodingProperties
 *    Windows.Media.MediaProperties.IMediaEncodingProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_TimedMetadataEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_TimedMetadataEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_TimedMetadataEncodingProperties[] = L"Windows.Media.MediaProperties.TimedMetadataEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.MediaProperties.VideoEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics3 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IVideoEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IVideoEncodingProperties2
 *    Windows.Media.MediaProperties.IVideoEncodingProperties3
 *    Windows.Media.MediaProperties.IVideoEncodingProperties4
 *    Windows.Media.MediaProperties.IVideoEncodingProperties5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_VideoEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_VideoEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_VideoEncodingProperties[] = L"Windows.Media.MediaProperties.VideoEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.Vp9ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IVp9ProfileIdsStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_Vp9ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_Vp9ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_Vp9ProfileIds[] = L"Windows.Media.MediaProperties.Vp9ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2 __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3 __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2 __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2 __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2 __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2 __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3 __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7 __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2 __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3 __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4 __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5 __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2 __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3 __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics;

#endif // ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_FWD_DEFINED__

// Parameterized interface forward declarations (C)

// Collection interface definitions

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile;

typedef struct __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfileVtbl;

interface __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* This,
        __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfileVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__
#define ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor;

#endif // ____x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

typedef struct __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl;

interface __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor
{
    CONST_VTBL struct __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

typedef struct __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        __FIIterator_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor** result);

    END_INTERFACE
} __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl;

interface __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor
{
    CONST_VTBL struct __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

typedef struct __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl;

interface __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor
{
    CONST_VTBL struct __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor;

typedef struct __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        __FIVectorView_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 index,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CMedia_CCore_CIMediaStreamDescriptor** items);

    END_INTERFACE
} __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl;

interface __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor
{
    CONST_VTBL struct __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFile __x_ABI_CWindows_CStorage_CIStorageFile;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality;

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat;

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat;

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CStereoscopicVideoPackingMode __x_ABI_CWindows_CMedia_CMediaProperties_CStereoscopicVideoPackingMode;

typedef enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality;

/*
 *
 * Struct Windows.Media.MediaProperties.AudioEncodingQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality
{
    AudioEncodingQuality_Auto = 0,
    AudioEncodingQuality_High = 1,
    AudioEncodingQuality_Medium = 2,
    AudioEncodingQuality_Low = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaMirroringOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaMirroringOptions
{
    MediaMirroringOptions_None = 0,
    MediaMirroringOptions_Horizontal = 0x1,
    MediaMirroringOptions_Vertical = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaPixelFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat
{
    MediaPixelFormat_Nv12 = 0,
    MediaPixelFormat_Bgra8 = 1,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
    MediaPixelFormat_P010 = 2,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaRotation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaRotation
{
    MediaRotation_None = 0,
    MediaRotation_Clockwise90Degrees = 1,
    MediaRotation_Clockwise180Degrees = 2,
    MediaRotation_Clockwise270Degrees = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.MediaThumbnailFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaThumbnailFormat
{
    MediaThumbnailFormat_Bmp = 0,
    MediaThumbnailFormat_Bgra8 = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Media.MediaProperties.SphericalVideoFrameFormat
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat
{
    SphericalVideoFrameFormat_None = 0,
    SphericalVideoFrameFormat_Unsupported = 1,
    SphericalVideoFrameFormat_Equirectangular = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Media.MediaProperties.StereoscopicVideoPackingMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CStereoscopicVideoPackingMode
{
    StereoscopicVideoPackingMode_None = 0,
    StereoscopicVideoPackingMode_SideBySide = 1,
    StereoscopicVideoPackingMode_TopBottom = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Media.MediaProperties.VideoEncodingQuality
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality
{
    VideoEncodingQuality_Auto = 0,
    VideoEncodingQuality_HD1080p = 1,
    VideoEncodingQuality_HD720p = 2,
    VideoEncodingQuality_Wvga = 3,
    VideoEncodingQuality_Ntsc = 4,
    VideoEncodingQuality_Pal = 5,
    VideoEncodingQuality_Vga = 6,
    VideoEncodingQuality_Qvga = 7,
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    VideoEncodingQuality_Uhd2160p = 8,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
    VideoEncodingQuality_Uhd4320p = 9,
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingProperties[] = L"Windows.Media.MediaProperties.IAudioEncodingProperties";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Bitrate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Bitrate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ChannelCount)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ChannelCount)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_SampleRate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_SampleRate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_BitsPerSample)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_BitsPerSample)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_put_Bitrate(This, value) \
    ((This)->lpVtbl->put_Bitrate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_get_Bitrate(This, value) \
    ((This)->lpVtbl->get_Bitrate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_put_ChannelCount(This, value) \
    ((This)->lpVtbl->put_ChannelCount(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_get_ChannelCount(This, value) \
    ((This)->lpVtbl->get_ChannelCount(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_put_SampleRate(This, value) \
    ((This)->lpVtbl->put_SampleRate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_get_SampleRate(This, value) \
    ((This)->lpVtbl->get_SampleRate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_put_BitsPerSample(This, value) \
    ((This)->lpVtbl->put_BitsPerSample(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_get_BitsPerSample(This, value) \
    ((This)->lpVtbl->get_BitsPerSample(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingProperties2[] = L"Windows.Media.MediaProperties.IAudioEncodingProperties2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsSpatial)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_get_IsSpatial(This, value) \
    ((This)->lpVtbl->get_IsSpatial(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingProperties3[] = L"Windows.Media.MediaProperties.IAudioEncodingProperties3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Copy)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_Copy(This, result) \
    ((This)->lpVtbl->Copy(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitrate,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateAacAdts)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitrate,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateMp3)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitrate,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreatePcm)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitsPerSample,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateWma)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitrate,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_CreateAac(This, sampleRate, channelCount, bitrate, value) \
    ((This)->lpVtbl->CreateAac(This, sampleRate, channelCount, bitrate, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_CreateAacAdts(This, sampleRate, channelCount, bitrate, value) \
    ((This)->lpVtbl->CreateAacAdts(This, sampleRate, channelCount, bitrate, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_CreateMp3(This, sampleRate, channelCount, bitrate, value) \
    ((This)->lpVtbl->CreateMp3(This, sampleRate, channelCount, bitrate, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_CreatePcm(This, sampleRate, channelCount, bitsPerSample, value) \
    ((This)->lpVtbl->CreatePcm(This, sampleRate, channelCount, bitsPerSample, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_CreateWma(This, sampleRate, channelCount, bitrate, value) \
    ((This)->lpVtbl->CreateWma(This, sampleRate, channelCount, bitrate, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingPropertiesStatics2[] = L"Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAlac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitsPerSample,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateFlac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2* This,
        UINT32 sampleRate,
        UINT32 channelCount,
        UINT32 bitsPerSample,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_CreateAlac(This, sampleRate, channelCount, bitsPerSample, value) \
    ((This)->lpVtbl->CreateAlac(This, sampleRate, channelCount, bitsPerSample, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_CreateFlac(This, sampleRate, channelCount, bitsPerSample, value) \
    ((This)->lpVtbl->CreateFlac(This, sampleRate, channelCount, bitsPerSample, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.AudioEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAudioEncodingPropertiesWithFormatUserData[] = L"Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetFormatUserData)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* GetFormatUserData)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData* This,
        UINT32* valueLength,
        BYTE** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserDataVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_SetFormatUserData(This, valueLength, value) \
    ((This)->lpVtbl->SetFormatUserData(This, valueLength, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_GetFormatUserData(This, valueLength, value) \
    ((This)->lpVtbl->GetFormatUserData(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingPropertiesWithFormatUserData_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IAv1ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.Av1ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IAv1ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IAv1ProfileIdsStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling420BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling420BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling400BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling400BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_HighChromaSubsampling444BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_HighChromaSubsampling444BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfessionalChromaSubsampling420BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfessionalChromaSubsampling400BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfessionalChromaSubsampling444BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfessionalChromaSubsampling422BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfessionalChromaSubsampling422BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ProfessionalChromaSubsampling422BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_MainChromaSubsampling420BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling420BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_MainChromaSubsampling420BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling420BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_MainChromaSubsampling400BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling400BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_MainChromaSubsampling400BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling400BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_HighChromaSubsampling444BitDepth8(This, value) \
    ((This)->lpVtbl->get_HighChromaSubsampling444BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_HighChromaSubsampling444BitDepth10(This, value) \
    ((This)->lpVtbl->get_HighChromaSubsampling444BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_ProfessionalChromaSubsampling420BitDepth12(This, value) \
    ((This)->lpVtbl->get_ProfessionalChromaSubsampling420BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_ProfessionalChromaSubsampling400BitDepth12(This, value) \
    ((This)->lpVtbl->get_ProfessionalChromaSubsampling400BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_ProfessionalChromaSubsampling444BitDepth12(This, value) \
    ((This)->lpVtbl->get_ProfessionalChromaSubsampling444BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_ProfessionalChromaSubsampling422BitDepth8(This, value) \
    ((This)->lpVtbl->get_ProfessionalChromaSubsampling422BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_ProfessionalChromaSubsampling422BitDepth10(This, value) \
    ((This)->lpVtbl->get_ProfessionalChromaSubsampling422BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_get_ProfessionalChromaSubsampling422BitDepth12(This, value) \
    ((This)->lpVtbl->get_ProfessionalChromaSubsampling422BitDepth12(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIAv1ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IContainerEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ContainerEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IContainerEncodingProperties[] = L"Windows.Media.MediaProperties.IContainerEncodingProperties";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* This,
        TrustLevel* trustLevel);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IContainerEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ContainerEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IContainerEncodingProperties2[] = L"Windows.Media.MediaProperties.IContainerEncodingProperties2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Copy)(__x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_Copy(This, result) \
    ((This)->lpVtbl->Copy(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IH264ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.H264ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IH264ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IH264ProfileIdsStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ConstrainedBaseline)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Baseline)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Extended)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Main)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_High)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_High10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_High422)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_High444)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_StereoHigh)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MultiviewHigh)(__x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_ConstrainedBaseline(This, value) \
    ((This)->lpVtbl->get_ConstrainedBaseline(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_Baseline(This, value) \
    ((This)->lpVtbl->get_Baseline(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_Extended(This, value) \
    ((This)->lpVtbl->get_Extended(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_Main(This, value) \
    ((This)->lpVtbl->get_Main(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_High(This, value) \
    ((This)->lpVtbl->get_High(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_High10(This, value) \
    ((This)->lpVtbl->get_High10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_High422(This, value) \
    ((This)->lpVtbl->get_High422(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_High444(This, value) \
    ((This)->lpVtbl->get_High444(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_StereoHigh(This, value) \
    ((This)->lpVtbl->get_StereoHigh(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_get_MultiviewHigh(This, value) \
    ((This)->lpVtbl->get_MultiviewHigh(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIH264ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IHevcProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.HevcProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IHevcProfileIdsStatics[] = L"Windows.Media.MediaProperties.IHevcProfileIdsStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling420BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling420BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling420BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling422BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling422BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling444BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling444BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainChromaSubsampling444BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MonochromeBitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MonochromeBitDepth16)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling420BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling420BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling420BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling422BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling422BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling444BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling444BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling444BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainIntraChromaSubsampling444BitDepth16)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainStillChromaSubsampling420BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainStillChromaSubsampling444BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_MainStillChromaSubsampling444BitDepth16)(__x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling420BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling420BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling420BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling420BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling420BitDepth12(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling420BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling422BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling422BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling422BitDepth12(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling422BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling444BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling444BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling444BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling444BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainChromaSubsampling444BitDepth12(This, value) \
    ((This)->lpVtbl->get_MainChromaSubsampling444BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MonochromeBitDepth12(This, value) \
    ((This)->lpVtbl->get_MonochromeBitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MonochromeBitDepth16(This, value) \
    ((This)->lpVtbl->get_MonochromeBitDepth16(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling420BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling420BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling420BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling420BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling420BitDepth12(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling420BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling422BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling422BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling422BitDepth12(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling422BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling444BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling444BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling444BitDepth10(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling444BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling444BitDepth12(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling444BitDepth12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainIntraChromaSubsampling444BitDepth16(This, value) \
    ((This)->lpVtbl->get_MainIntraChromaSubsampling444BitDepth16(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainStillChromaSubsampling420BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainStillChromaSubsampling420BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainStillChromaSubsampling444BitDepth8(This, value) \
    ((This)->lpVtbl->get_MainStillChromaSubsampling444BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_get_MainStillChromaSubsampling444BitDepth16(This, value) \
    ((This)->lpVtbl->get_MainStillChromaSubsampling444BitDepth16(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIHevcProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingProperties[] = L"Windows.Media.MediaProperties.IImageEncodingProperties";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Width)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Height)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_put_Width(This, value) \
    ((This)->lpVtbl->put_Width(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_put_Height(This, value) \
    ((This)->lpVtbl->put_Height(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingProperties2[] = L"Windows.Media.MediaProperties.IImageEncodingProperties2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Copy)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_Copy(This, result) \
    ((This)->lpVtbl->Copy(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.IImageEncodingPropertiesStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateJpeg)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreatePng)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateJpegXR)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_CreateJpeg(This, value) \
    ((This)->lpVtbl->CreateJpeg(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_CreatePng(This, value) \
    ((This)->lpVtbl->CreatePng(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_CreateJpegXR(This, value) \
    ((This)->lpVtbl->CreateJpegXR(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingPropertiesStatics2[] = L"Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateUncompressed)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CMediaPixelFormat format,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateBmp)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_CreateUncompressed(This, format, value) \
    ((This)->lpVtbl->CreateUncompressed(This, format, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_CreateBmp(This, value) \
    ((This)->lpVtbl->CreateBmp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.ImageEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IImageEncodingPropertiesStatics3[] = L"Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateHeif)(__x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_CreateHeif(This, result) \
    ((This)->lpVtbl->CreateHeif(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIImageEncodingPropertiesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfile[] = L"Windows.Media.MediaProperties.IMediaEncodingProfile";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Audio)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties* value);
    HRESULT (STDMETHODCALLTYPE* get_Audio)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIAudioEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* put_Video)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* value);
    HRESULT (STDMETHODCALLTYPE* get_Video)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* put_Container)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties* value);
    HRESULT (STDMETHODCALLTYPE* get_Container)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIContainerEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_put_Audio(This, value) \
    ((This)->lpVtbl->put_Audio(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_get_Audio(This, value) \
    ((This)->lpVtbl->get_Audio(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_put_Video(This, value) \
    ((This)->lpVtbl->put_Video(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_get_Video(This, value) \
    ((This)->lpVtbl->get_Video(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_put_Container(This, value) \
    ((This)->lpVtbl->put_Container(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_get_Container(This, value) \
    ((This)->lpVtbl->get_Container(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfile2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfile2[] = L"Windows.Media.MediaProperties.IMediaEncodingProfile2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetAudioTracks)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        __FIIterable_1_Windows__CMedia__CCore__CAudioStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* GetAudioTracks)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        __FIVector_1_Windows__CMedia__CCore__CAudioStreamDescriptor** value);
    HRESULT (STDMETHODCALLTYPE* SetVideoTracks)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        __FIIterable_1_Windows__CMedia__CCore__CVideoStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* GetVideoTracks)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2* This,
        __FIVector_1_Windows__CMedia__CCore__CVideoStreamDescriptor** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_SetAudioTracks(This, value) \
    ((This)->lpVtbl->SetAudioTracks(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_GetAudioTracks(This, value) \
    ((This)->lpVtbl->GetAudioTracks(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_SetVideoTracks(This, value) \
    ((This)->lpVtbl->SetVideoTracks(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_GetVideoTracks(This, value) \
    ((This)->lpVtbl->GetVideoTracks(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfile3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfile3[] = L"Windows.Media.MediaProperties.IMediaEncodingProfile3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetTimedMetadataTracks)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This,
        __FIIterable_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor* value);
    HRESULT (STDMETHODCALLTYPE* GetTimedMetadataTracks)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3* This,
        __FIVector_1_Windows__CMedia__CCore__CTimedMetadataStreamDescriptor** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_SetTimedMetadataTracks(This, value) \
    ((This)->lpVtbl->SetTimedMetadataTracks(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_GetTimedMetadataTracks(This, result) \
    ((This)->lpVtbl->GetTimedMetadataTracks(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateM4a)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateMp3)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateWma)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateMp4)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateWmv)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateFromFileAsync)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        __x_ABI_CWindows_CStorage_CIStorageFile* file,
        __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile** operation);
    HRESULT (STDMETHODCALLTYPE* CreateFromStreamAsync)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics* This,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStream* stream,
        __FIAsyncOperation_1_Windows__CMedia__CMediaProperties__CMediaEncodingProfile** operation);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateM4a(This, quality, value) \
    ((This)->lpVtbl->CreateM4a(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateMp3(This, quality, value) \
    ((This)->lpVtbl->CreateMp3(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateWma(This, quality, value) \
    ((This)->lpVtbl->CreateWma(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateMp4(This, quality, value) \
    ((This)->lpVtbl->CreateMp4(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateWmv(This, quality, value) \
    ((This)->lpVtbl->CreateWmv(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateFromFileAsync(This, file, operation) \
    ((This)->lpVtbl->CreateFromFileAsync(This, file, operation))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_CreateFromStreamAsync(This, stream, operation) \
    ((This)->lpVtbl->CreateFromStreamAsync(This, stream, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics2[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateWav)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateAvi)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_CreateWav(This, quality, value) \
    ((This)->lpVtbl->CreateWav(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_CreateAvi(This, quality, value) \
    ((This)->lpVtbl->CreateAvi(This, quality, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics3[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateAlac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateFlac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CAudioEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);
    HRESULT (STDMETHODCALLTYPE* CreateHevc)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_CreateAlac(This, quality, value) \
    ((This)->lpVtbl->CreateAlac(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_CreateFlac(This, quality, value) \
    ((This)->lpVtbl->CreateFlac(This, quality, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_CreateHevc(This, quality, value) \
    ((This)->lpVtbl->CreateHevc(This, quality, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProfileStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingProfile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProfileStatics4[] = L"Windows.Media.MediaProperties.IMediaEncodingProfileStatics4";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVp9)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** result);
    HRESULT (STDMETHODCALLTYPE* CreateAv1)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CVideoEncodingQuality quality,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfile** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_CreateVp9(This, quality, result) \
    ((This)->lpVtbl->CreateVp9(This, quality, result))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_CreateAv1(This, quality, result) \
    ((This)->lpVtbl->CreateAv1(This, quality, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProfileStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingProperties[] = L"Windows.Media.MediaProperties.IMediaEncodingProperties";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        __FIMap_2_GUID_IInspectable** value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Subtype)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Subtype)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_put_Subtype(This, value) \
    ((This)->lpVtbl->put_Subtype(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_get_Subtype(This, value) \
    ((This)->lpVtbl->get_Subtype(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Aac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AacAdts)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ac3)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AmrNb)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AmrWb)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Argb32)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Asf)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Avi)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Bgra8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Bmp)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Eac3)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Float)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Gif)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_H263)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_H264)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_H264Es)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Hevc)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_HevcEs)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Iyuv)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Jpeg)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_JpegXr)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mjpg)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mpeg)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mpeg1)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mpeg2)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mp3)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Mpeg4)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Nv12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Pcm)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Png)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rgb24)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Rgb32)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Tiff)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wave)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wma8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wma9)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wmv3)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Wvc1)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Yuy2)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Yv12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Aac(This, value) \
    ((This)->lpVtbl->get_Aac(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_AacAdts(This, value) \
    ((This)->lpVtbl->get_AacAdts(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Ac3(This, value) \
    ((This)->lpVtbl->get_Ac3(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_AmrNb(This, value) \
    ((This)->lpVtbl->get_AmrNb(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_AmrWb(This, value) \
    ((This)->lpVtbl->get_AmrWb(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Argb32(This, value) \
    ((This)->lpVtbl->get_Argb32(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Asf(This, value) \
    ((This)->lpVtbl->get_Asf(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Avi(This, value) \
    ((This)->lpVtbl->get_Avi(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Bgra8(This, value) \
    ((This)->lpVtbl->get_Bgra8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Bmp(This, value) \
    ((This)->lpVtbl->get_Bmp(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Eac3(This, value) \
    ((This)->lpVtbl->get_Eac3(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Float(This, value) \
    ((This)->lpVtbl->get_Float(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Gif(This, value) \
    ((This)->lpVtbl->get_Gif(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_H263(This, value) \
    ((This)->lpVtbl->get_H263(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_H264(This, value) \
    ((This)->lpVtbl->get_H264(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_H264Es(This, value) \
    ((This)->lpVtbl->get_H264Es(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Hevc(This, value) \
    ((This)->lpVtbl->get_Hevc(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_HevcEs(This, value) \
    ((This)->lpVtbl->get_HevcEs(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Iyuv(This, value) \
    ((This)->lpVtbl->get_Iyuv(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Jpeg(This, value) \
    ((This)->lpVtbl->get_Jpeg(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_JpegXr(This, value) \
    ((This)->lpVtbl->get_JpegXr(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Mjpg(This, value) \
    ((This)->lpVtbl->get_Mjpg(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Mpeg(This, value) \
    ((This)->lpVtbl->get_Mpeg(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Mpeg1(This, value) \
    ((This)->lpVtbl->get_Mpeg1(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Mpeg2(This, value) \
    ((This)->lpVtbl->get_Mpeg2(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Mp3(This, value) \
    ((This)->lpVtbl->get_Mp3(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Mpeg4(This, value) \
    ((This)->lpVtbl->get_Mpeg4(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Nv12(This, value) \
    ((This)->lpVtbl->get_Nv12(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Pcm(This, value) \
    ((This)->lpVtbl->get_Pcm(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Png(This, value) \
    ((This)->lpVtbl->get_Png(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Rgb24(This, value) \
    ((This)->lpVtbl->get_Rgb24(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Rgb32(This, value) \
    ((This)->lpVtbl->get_Rgb32(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Tiff(This, value) \
    ((This)->lpVtbl->get_Tiff(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Wave(This, value) \
    ((This)->lpVtbl->get_Wave(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Wma8(This, value) \
    ((This)->lpVtbl->get_Wma8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Wma9(This, value) \
    ((This)->lpVtbl->get_Wma9(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Wmv3(This, value) \
    ((This)->lpVtbl->get_Wmv3(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Wvc1(This, value) \
    ((This)->lpVtbl->get_Wvc1(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Yuy2(This, value) \
    ((This)->lpVtbl->get_Yuy2(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_get_Yv12(This, value) \
    ((This)->lpVtbl->get_Yv12(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics2[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Vp9)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_L8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_L16)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_D16)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_get_Vp9(This, value) \
    ((This)->lpVtbl->get_Vp9(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_get_L8(This, value) \
    ((This)->lpVtbl->get_L8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_get_L16(This, value) \
    ((This)->lpVtbl->get_L16(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_get_D16(This, value) \
    ((This)->lpVtbl->get_D16(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics3[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Alac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Flac)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_get_Alac(This, value) \
    ((This)->lpVtbl->get_Alac(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_get_Flac(This, value) \
    ((This)->lpVtbl->get_Flac(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics4[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_P010)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_get_P010(This, value) \
    ((This)->lpVtbl->get_P010(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics5[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Heif)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_get_Heif(This, value) \
    ((This)->lpVtbl->get_Heif(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics6[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Pgs)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Srt)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Ssa)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_VobSub)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_get_Pgs(This, value) \
    ((This)->lpVtbl->get_Pgs(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_get_Srt(This, value) \
    ((This)->lpVtbl->get_Srt(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_get_Ssa(This, value) \
    ((This)->lpVtbl->get_Ssa(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_get_VobSub(This, value) \
    ((This)->lpVtbl->get_VobSub(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaEncodingSubtypesStatics7[] = L"Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics7";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Av1)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_get_Av1(This, value) \
    ((This)->lpVtbl->get_Av1(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingSubtypesStatics7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IMediaRatio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.MediaRatio
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMediaRatio[] = L"Windows.Media.MediaProperties.IMediaRatio";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatioVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Numerator)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Numerator)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Denominator)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Denominator)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatioVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatioVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_put_Numerator(This, value) \
    ((This)->lpVtbl->put_Numerator(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_get_Numerator(This, value) \
    ((This)->lpVtbl->get_Numerator(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_put_Denominator(This, value) \
    ((This)->lpVtbl->put_Denominator(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_get_Denominator(This, value) \
    ((This)->lpVtbl->get_Denominator(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.Mpeg2ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IMpeg2ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Simple)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Main)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SignalNoiseRatioScalable)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_SpatiallyScalable)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_High)(__x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_get_Simple(This, value) \
    ((This)->lpVtbl->get_Simple(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_get_Main(This, value) \
    ((This)->lpVtbl->get_Main(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_get_SignalNoiseRatioScalable(This, value) \
    ((This)->lpVtbl->get_SignalNoiseRatioScalable(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_get_SpatiallyScalable(This, value) \
    ((This)->lpVtbl->get_SpatiallyScalable(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_get_High(This, value) \
    ((This)->lpVtbl->get_High(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIMpeg2ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.ITimedMetadataEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.TimedMetadataEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_ITimedMetadataEncodingProperties[] = L"Windows.Media.MediaProperties.ITimedMetadataEncodingProperties";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetFormatUserData)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* GetFormatUserData)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* Copy)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_SetFormatUserData(This, valueLength, value) \
    ((This)->lpVtbl->SetFormatUserData(This, valueLength, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_GetFormatUserData(This, valueLength, value) \
    ((This)->lpVtbl->GetFormatUserData(This, valueLength, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_Copy(This, result) \
    ((This)->lpVtbl->Copy(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.TimedMetadataEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_ITimedMetadataEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreatePgs)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* CreateSrt)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* CreateSsa)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        UINT32 formatUserDataLength,
        BYTE* formatUserData,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* CreateVobSub)(__x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics* This,
        UINT32 formatUserDataLength,
        BYTE* formatUserData,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_CreatePgs(This, result) \
    ((This)->lpVtbl->CreatePgs(This, result))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_CreateSrt(This, result) \
    ((This)->lpVtbl->CreateSrt(This, result))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_CreateSsa(This, formatUserDataLength, formatUserData, result) \
    ((This)->lpVtbl->CreateSsa(This, formatUserDataLength, formatUserData, result))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_CreateVobSub(This, formatUserDataLength, formatUserData, result) \
    ((This)->lpVtbl->CreateVobSub(This, formatUserDataLength, formatUserData, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CITimedMetadataEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 * Any object which implements this interface must also implement the following interfaces:
 *     Windows.Media.MediaProperties.IMediaEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Bitrate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Bitrate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Width)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Width)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* put_Height)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        UINT32 value);
    HRESULT (STDMETHODCALLTYPE* get_Height)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_FrameRate)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);
    HRESULT (STDMETHODCALLTYPE* get_PixelAspectRatio)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIMediaRatio** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_put_Bitrate(This, value) \
    ((This)->lpVtbl->put_Bitrate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_get_Bitrate(This, value) \
    ((This)->lpVtbl->get_Bitrate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_put_Width(This, value) \
    ((This)->lpVtbl->put_Width(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_get_Width(This, value) \
    ((This)->lpVtbl->get_Width(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_put_Height(This, value) \
    ((This)->lpVtbl->put_Height(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_get_Height(This, value) \
    ((This)->lpVtbl->get_Height(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_get_FrameRate(This, value) \
    ((This)->lpVtbl->get_FrameRate(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_get_PixelAspectRatio(This, value) \
    ((This)->lpVtbl->get_PixelAspectRatio(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties2[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* SetFormatUserData)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        UINT32 valueLength,
        BYTE* value);
    HRESULT (STDMETHODCALLTYPE* GetFormatUserData)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        UINT32* valueLength,
        BYTE** value);
    HRESULT (STDMETHODCALLTYPE* put_ProfileId)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ProfileId)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_SetFormatUserData(This, valueLength, value) \
    ((This)->lpVtbl->SetFormatUserData(This, valueLength, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_GetFormatUserData(This, valueLength, value) \
    ((This)->lpVtbl->GetFormatUserData(This, valueLength, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_put_ProfileId(This, value) \
    ((This)->lpVtbl->put_ProfileId(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_get_ProfileId(This, value) \
    ((This)->lpVtbl->get_ProfileId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties3[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_StereoscopicVideoPackingMode)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CStereoscopicVideoPackingMode* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_get_StereoscopicVideoPackingMode(This, value) \
    ((This)->lpVtbl->get_StereoscopicVideoPackingMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties4[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties4";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SphericalVideoFrameFormat)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4* This,
        enum __x_ABI_CWindows_CMedia_CMediaProperties_CSphericalVideoFrameFormat* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_get_SphericalVideoFrameFormat(This, value) \
    ((This)->lpVtbl->get_SphericalVideoFrameFormat(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingProperties5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingProperties5[] = L"Windows.Media.MediaProperties.IVideoEncodingProperties5";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Copy)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_Copy(This, result) \
    ((This)->lpVtbl->Copy(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingPropertiesStatics[] = L"Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateH264)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateMpeg2)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** value);
    HRESULT (STDMETHODCALLTYPE* CreateUncompressed)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics* This,
        HSTRING subtype,
        UINT32 width,
        UINT32 height,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_CreateH264(This, value) \
    ((This)->lpVtbl->CreateH264(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_CreateMpeg2(This, value) \
    ((This)->lpVtbl->CreateMpeg2(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_CreateUncompressed(This, subtype, width, height, value) \
    ((This)->lpVtbl->CreateUncompressed(This, subtype, width, height, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingPropertiesStatics2[] = L"Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateHevc)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_CreateHevc(This, value) \
    ((This)->lpVtbl->CreateHevc(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.VideoEncodingProperties
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVideoEncodingPropertiesStatics3[] = L"Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics3";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateVp9)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** result);
    HRESULT (STDMETHODCALLTYPE* CreateAv1)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3* This,
        __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingProperties** result);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3Vtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_CreateVp9(This, result) \
    ((This)->lpVtbl->CreateVp9(This, result))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_CreateAv1(This, result) \
    ((This)->lpVtbl->CreateAv1(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVideoEncodingPropertiesStatics3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.Media.MediaProperties.IVp9ProfileIdsStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.Media.MediaProperties.Vp9ProfileIds
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Media_MediaProperties_IVp9ProfileIdsStatics[] = L"Windows.Media.MediaProperties.IVp9ProfileIdsStatics";
typedef struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Profile0ChromaSubsampling420BitDepth8)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Profile2ChromaSubsampling420BitDepth10)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Profile2ChromaSubsampling420BitDepth12)(__x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics* This,
        INT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStaticsVtbl;

interface __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_get_Profile0ChromaSubsampling420BitDepth8(This, value) \
    ((This)->lpVtbl->get_Profile0ChromaSubsampling420BitDepth8(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_get_Profile2ChromaSubsampling420BitDepth10(This, value) \
    ((This)->lpVtbl->get_Profile2ChromaSubsampling420BitDepth10(This, value))

#define __x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_get_Profile2ChromaSubsampling420BitDepth12(This, value) \
    ((This)->lpVtbl->get_Profile2ChromaSubsampling420BitDepth12(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics;
#endif /* !defined(____x_ABI_CWindows_CMedia_CMediaProperties_CIVp9ProfileIdsStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Media.MediaProperties.AudioEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IAudioEncodingPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IAudioEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IAudioEncodingPropertiesWithFormatUserData
 *    Windows.Media.MediaProperties.IAudioEncodingProperties2
 *    Windows.Media.MediaProperties.IAudioEncodingProperties3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_AudioEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_AudioEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_AudioEncodingProperties[] = L"Windows.Media.MediaProperties.AudioEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.Av1ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IAv1ProfileIdsStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_Av1ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_Av1ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_Av1ProfileIds[] = L"Windows.Media.MediaProperties.Av1ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Media.MediaProperties.ContainerEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IContainerEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IContainerEncodingProperties2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_ContainerEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_ContainerEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_ContainerEncodingProperties[] = L"Windows.Media.MediaProperties.ContainerEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.H264ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IH264ProfileIdsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_H264ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_H264ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_H264ProfileIds[] = L"Windows.Media.MediaProperties.H264ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.HevcProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IHevcProfileIdsStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_HevcProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_HevcProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_HevcProfileIds[] = L"Windows.Media.MediaProperties.HevcProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.Media.MediaProperties.ImageEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IImageEncodingPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IImageEncodingPropertiesStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IImageEncodingPropertiesStatics3 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IImageEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IImageEncodingProperties2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_ImageEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_ImageEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_ImageEncodingProperties[] = L"Windows.Media.MediaProperties.ImageEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaEncodingProfile
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics4 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingProfileStatics2 interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IMediaEncodingProfile ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProfile2
 *    Windows.Media.MediaProperties.IMediaEncodingProfile3
 *
 * Class Threading Model:  Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingProfile_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingProfile_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaEncodingProfile[] = L"Windows.Media.MediaProperties.MediaEncodingProfile";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaEncodingSubtypes
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics3 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics5 interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics7 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics4 interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics6 interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IMediaEncodingSubtypesStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingSubtypes_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaEncodingSubtypes_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaEncodingSubtypes[] = L"Windows.Media.MediaProperties.MediaEncodingSubtypes";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaPropertySet
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Foundation.Collections.IMap`2<Guid, Object> ** Default Interface **
 *    Windows.Foundation.Collections.IIterable`1<Windows.Foundation.Collections.IKeyValuePair`2<Guid, Object>>
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaPropertySet_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaPropertySet_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaPropertySet[] = L"Windows.Media.MediaProperties.MediaPropertySet";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.MediaRatio
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IMediaRatio ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_MediaRatio_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_MediaRatio_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_MediaRatio[] = L"Windows.Media.MediaProperties.MediaRatio";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.Mpeg2ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IMpeg2ProfileIdsStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_Mpeg2ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_Mpeg2ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_Mpeg2ProfileIds[] = L"Windows.Media.MediaProperties.Mpeg2ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.TimedMetadataEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.ITimedMetadataEncodingPropertiesStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.ITimedMetadataEncodingProperties
 *    Windows.Media.MediaProperties.IMediaEncodingProperties ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_TimedMetadataEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_TimedMetadataEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_TimedMetadataEncodingProperties[] = L"Windows.Media.MediaProperties.TimedMetadataEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Media.MediaProperties.VideoEncodingProperties
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics3 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics2 interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.Media.MediaProperties.IVideoEncodingPropertiesStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Media.MediaProperties.IVideoEncodingProperties ** Default Interface **
 *    Windows.Media.MediaProperties.IMediaEncodingProperties
 *    Windows.Media.MediaProperties.IVideoEncodingProperties2
 *    Windows.Media.MediaProperties.IVideoEncodingProperties3
 *    Windows.Media.MediaProperties.IVideoEncodingProperties4
 *    Windows.Media.MediaProperties.IVideoEncodingProperties5
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_VideoEncodingProperties_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_VideoEncodingProperties_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_VideoEncodingProperties[] = L"Windows.Media.MediaProperties.VideoEncodingProperties";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Media.MediaProperties.Vp9ProfileIds
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Media.MediaProperties.IVp9ProfileIdsStatics interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_Media_MediaProperties_Vp9ProfileIds_DEFINED
#define RUNTIMECLASS_Windows_Media_MediaProperties_Vp9ProfileIds_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Media_MediaProperties_Vp9ProfileIds[] = L"Windows.Media.MediaProperties.Vp9ProfileIds";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emedia2Emediaproperties_p_h__

#endif // __windows2Emedia2Emediaproperties_h__
