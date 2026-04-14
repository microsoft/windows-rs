
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
#ifndef __windows2Esecurity2Eisolation_h__
#define __windows2Esecurity2Eisolation_h__
#ifndef __windows2Esecurity2Eisolation_p_h__
#define __windows2Esecurity2Eisolation_p_h__


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

#if !defined(WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION)
#define WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION 0x50000
#endif // defined(WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IHostMessageReceivedCallback;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback ABI::Windows::Security::Isolation::IHostMessageReceivedCallback

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IMessageReceivedCallback;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback ABI::Windows::Security::Isolation::IMessageReceivedCallback

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironment;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironment2;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment2

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironment3;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment3

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironment4;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment4

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentCreateResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentCreateResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentCreateResult2;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentCreateResult2

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentFactory;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentFactory

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentFile;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentFile

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentFile2;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentFile2

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentHostStatics;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentHostStatics

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentLaunchFileResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentLaunchFileResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentOptions;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOptions

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentOptions2;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOptions2

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentOptions3;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOptions3

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentOwnerRegistrationData;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOwnerRegistrationData

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentOwnerRegistrationResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOwnerRegistrationResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentOwnerRegistrationStatics;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOwnerRegistrationStatics

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentPostMessageResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentPostMessageResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentProcess;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentProcess

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentShareFileRequestOptions;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFileRequestOptions

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentShareFileResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFileResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentShareFolderRequestOptions;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFolderRequestOptions

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentShareFolderResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFolderResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentStartProcessResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentStartProcessResult

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentTelemetryParameters;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentUserInfo;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentUserInfo

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsEnvironmentUserInfo2;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2 ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentUserInfo2

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsHostMessengerStatics;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics ABI::Windows::Security::Isolation::IIsolatedWindowsHostMessengerStatics

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                interface IIsolatedWindowsHostMessengerStatics2;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2 ABI::Windows::Security::Isolation::IIsolatedWindowsHostMessengerStatics2

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_FWD_DEFINED__

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
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentLaunchFileResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("af891fa2-3f89-5e5b-a72b-df936bb1ad45"))
IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentLaunchFileResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileResult*> __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("940ee865-8c8c-5b6b-88f6-a0371f58b386"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentLaunchFileResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentPostMessageResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3e167db0-7983-5329-83a0-0477763cc4cb"))
IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentPostMessageResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageResult*> __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("34a084e3-dea5-5e5b-8d86-a31584e74893"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentPostMessageResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentShareFileResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("76ce76bf-ec05-513d-843f-79af1766e490"))
IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFileResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileResult*> __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("8baca361-2935-5255-84dc-1c854f4fef69"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFileResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentShareFolderResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("89424ded-b21a-55ad-9d4a-8e53f15464a2"))
IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFolderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderResult*> __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("555c4bca-d35b-5e94-b8e5-a7e3c44bd1c1"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFolderResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentStartProcessResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_USE
#define DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7ccbc39f-2baa-5747-a988-ffa38f14a446"))
IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentStartProcessResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessResult*> __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_t;
#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("0687867c-5e3f-53bb-91c0-cc931948d524"))
IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentStartProcessResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessResult*> __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentSignInProgress : int IsolatedWindowsEnvironmentSignInProgress;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e55295a4-d669-5922-bf42-bbe7eaeb4596"))
IAsyncOperationWithProgressCompletedHandler<bool, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Boolean, Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<bool, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress> __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("3342214c-85a9-5729-b0f1-fc667640078a"))
IAsyncOperationWithProgress<bool, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Boolean, Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<bool, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress> __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_t;
#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("75620fb8-3a60-5676-8dd6-d8f700a2fcdc"))
IAsyncOperationProgressHandler<bool, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<bool, boolean>, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Boolean, Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<bool, enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentSignInProgress> __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_t;
#define __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentCreateResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef struct IsolatedWindowsEnvironmentCreateProgress IsolatedWindowsEnvironmentCreateProgress;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6c4e60b8-ed93-5e10-96c5-fafadb68ff31"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentCreateResult*>, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult, Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df32b4b4-4508-594a-830b-b46ced2a4ae0"))
IAsyncOperationWithProgress<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentCreateResult*>, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult, Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress> __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("417f9026-1750-5e97-b6c9-fb2dadd61a0b"))
IAsyncOperationProgressHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentCreateResult*>, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult, Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateResult*, struct ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateProgress> __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIIterator_1_IInspectable_USE
#define DEF___FIIterator_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("44a94f2d-04f8-5091-b336-be7892dd10be"))
IIterator<IInspectable*> : IIterator_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<IInspectable*> __FIIterator_1_IInspectable_t;
#define __FIIterator_1_IInspectable ABI::Windows::Foundation::Collections::__FIIterator_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_IInspectable_USE */



#ifndef DEF___FIIterable_1_IInspectable_USE
#define DEF___FIIterable_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("092b849b-60b1-52be-a44a-6fe8e933cbe4"))
IIterable<IInspectable*> : IIterable_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<IInspectable*> __FIIterable_1_IInspectable_t;
#define __FIIterable_1_IInspectable ABI::Windows::Foundation::Collections::__FIIterable_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_IInspectable_USE */



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
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironment;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE
#define DEF___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("db2f28c7-4c44-5c53-b01b-4381a8ce9800"))
IIterator<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Isolation.IsolatedWindowsEnvironment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*> __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_t;
#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE
#define DEF___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("37505de7-601e-5c17-a7f5-1a6ac4b91b03"))
IIterable<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Isolation.IsolatedWindowsEnvironment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*> __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_t;
#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentHostError : int IsolatedWindowsEnvironmentHostError;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE
#define DEF___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("63675c12-3214-5510-82d9-cb807ff89bc8"))
IIterator<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError> : IIterator_impl<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError> __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_t;
#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE
#define DEF___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e2c0cc79-8015-5196-8b32-7a0887a091bd"))
IIterable<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError> : IIterable_impl<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError> __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_t;
#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000


#ifndef DEF___FIVectorView_1_IInspectable_USE
#define DEF___FIVectorView_1_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a6487363-b074-5c60-ab16-866dce4ee54d"))
IVectorView<IInspectable*> : IVectorView_impl<IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<IInspectable*> __FIVectorView_1_IInspectable_t;
#define __FIVectorView_1_IInspectable ABI::Windows::Foundation::Collections::__FIVectorView_1_IInspectable_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_IInspectable_USE */



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


#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("74f0ee08-295b-5b41-93cb-10c4b7a7a56c"))
IVectorView<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*, ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Isolation.IsolatedWindowsEnvironment>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Security::Isolation::IsolatedWindowsEnvironment*> __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_t;
#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE
#define DEF___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a830f621-a4ae-5f5c-a003-985fe3e01e51"))
IVectorView<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError> : IVectorView_impl<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentHostError> __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_t;
#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_USE */

#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000


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

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentActivator : int IsolatedWindowsEnvironmentActivator;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentAllowedClipboardFormats : unsigned int IsolatedWindowsEnvironmentAllowedClipboardFormats;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentAvailablePrinters : unsigned int IsolatedWindowsEnvironmentAvailablePrinters;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentClipboardCopyPasteDirections : unsigned int IsolatedWindowsEnvironmentClipboardCopyPasteDirections;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentCreateStatus : int IsolatedWindowsEnvironmentCreateStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentCreationPriority : int IsolatedWindowsEnvironmentCreationPriority;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentLaunchFileStatus : int IsolatedWindowsEnvironmentLaunchFileStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentOwnerRegistrationStatus : int IsolatedWindowsEnvironmentOwnerRegistrationStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentPostMessageStatus : int IsolatedWindowsEnvironmentPostMessageStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentProcessState : int IsolatedWindowsEnvironmentProcessState;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentProgressState : int IsolatedWindowsEnvironmentProgressState;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentShareFileStatus : int IsolatedWindowsEnvironmentShareFileStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentShareFolderStatus : int IsolatedWindowsEnvironmentShareFolderStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                typedef enum IsolatedWindowsEnvironmentStartProcessStatus : int IsolatedWindowsEnvironmentStartProcessStatus;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentFile;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentOptions;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentOwnerRegistrationData;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentOwnerRegistrationResult;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentProcess;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentShareFileRequestOptions;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentShareFolderRequestOptions;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentTelemetryParameters;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                class IsolatedWindowsEnvironmentUserInfo;
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentActivator is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentActivator : int
                {
                    IsolatedWindowsEnvironmentActivator_System = 0,
                    IsolatedWindowsEnvironmentActivator_User = 1,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentAllowedClipboardFormats is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentAllowedClipboardFormats : unsigned int
                {
                    IsolatedWindowsEnvironmentAllowedClipboardFormats_None = 0,
                    IsolatedWindowsEnvironmentAllowedClipboardFormats_Text = 0x1,
                    IsolatedWindowsEnvironmentAllowedClipboardFormats_Image = 0x2,
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                    IsolatedWindowsEnvironmentAllowedClipboardFormats_Rtf = 0x4,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                };

                DEFINE_ENUM_FLAG_OPERATORS(IsolatedWindowsEnvironmentAllowedClipboardFormats)
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentAvailablePrinters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentAvailablePrinters : unsigned int
                {
                    IsolatedWindowsEnvironmentAvailablePrinters_None = 0,
                    IsolatedWindowsEnvironmentAvailablePrinters_Local = 0x1,
                    IsolatedWindowsEnvironmentAvailablePrinters_Network = 0x2,
                    IsolatedWindowsEnvironmentAvailablePrinters_SystemPrintToPdf = 0x4,
                    IsolatedWindowsEnvironmentAvailablePrinters_SystemPrintToXps = 0x8,
                };

                DEFINE_ENUM_FLAG_OPERATORS(IsolatedWindowsEnvironmentAvailablePrinters)
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentClipboardCopyPasteDirections is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentClipboardCopyPasteDirections : unsigned int
                {
                    IsolatedWindowsEnvironmentClipboardCopyPasteDirections_None = 0,
                    IsolatedWindowsEnvironmentClipboardCopyPasteDirections_HostToIsolatedWindowsEnvironment = 0x1,
                    IsolatedWindowsEnvironmentClipboardCopyPasteDirections_IsolatedWindowsEnvironmentToHost = 0x2,
                };

                DEFINE_ENUM_FLAG_OPERATORS(IsolatedWindowsEnvironmentClipboardCopyPasteDirections)
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentCreateStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentCreateStatus : int
                {
                    IsolatedWindowsEnvironmentCreateStatus_Success = 0,
                    IsolatedWindowsEnvironmentCreateStatus_FailureByPolicy = 1,
                    IsolatedWindowsEnvironmentCreateStatus_UnknownFailure = 2,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentCreationPriority
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentCreationPriority is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentCreationPriority : int
                {
                    IsolatedWindowsEnvironmentCreationPriority_Low = 0,
                    IsolatedWindowsEnvironmentCreationPriority_Normal = 1,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentHostError is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentHostError : int
                {
                    IsolatedWindowsEnvironmentHostError_AdminPolicyIsDisabledOrNotPresent = 0,
                    IsolatedWindowsEnvironmentHostError_FeatureNotInstalled = 1,
                    IsolatedWindowsEnvironmentHostError_HardwareRequirementsNotMet = 2,
                    IsolatedWindowsEnvironmentHostError_RebootRequired = 3,
                    IsolatedWindowsEnvironmentHostError_UnknownError = 4,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentLaunchFileStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentLaunchFileStatus : int
                {
                    IsolatedWindowsEnvironmentLaunchFileStatus_Success = 0,
                    IsolatedWindowsEnvironmentLaunchFileStatus_UnknownFailure = 1,
                    IsolatedWindowsEnvironmentLaunchFileStatus_EnvironmentUnavailable = 2,
                    IsolatedWindowsEnvironmentLaunchFileStatus_FileNotFound = 3,
                    IsolatedWindowsEnvironmentLaunchFileStatus_TimedOut = 4,
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
                    IsolatedWindowsEnvironmentLaunchFileStatus_AlreadySharedWithConflictingOptions = 5,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentOwnerRegistrationStatus : int
                {
                    IsolatedWindowsEnvironmentOwnerRegistrationStatus_Success = 0,
                    IsolatedWindowsEnvironmentOwnerRegistrationStatus_InvalidArgument = 1,
                    IsolatedWindowsEnvironmentOwnerRegistrationStatus_AccessDenied = 2,
                    IsolatedWindowsEnvironmentOwnerRegistrationStatus_InsufficientMemory = 3,
                    IsolatedWindowsEnvironmentOwnerRegistrationStatus_UnknownFailure = 4,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentPostMessageStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentPostMessageStatus : int
                {
                    IsolatedWindowsEnvironmentPostMessageStatus_Success = 0,
                    IsolatedWindowsEnvironmentPostMessageStatus_UnknownFailure = 1,
                    IsolatedWindowsEnvironmentPostMessageStatus_EnvironmentUnavailable = 2,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentProcessState is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentProcessState : int
                {
                    IsolatedWindowsEnvironmentProcessState_Running = 1,
                    IsolatedWindowsEnvironmentProcessState_Aborted = 2,
                    IsolatedWindowsEnvironmentProcessState_Completed = 3,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentProgressState is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentProgressState : int
                {
                    IsolatedWindowsEnvironmentProgressState_Queued = 0,
                    IsolatedWindowsEnvironmentProgressState_Processing = 1,
                    IsolatedWindowsEnvironmentProgressState_Completed = 2,
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                    IsolatedWindowsEnvironmentProgressState_Creating = 3,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                    IsolatedWindowsEnvironmentProgressState_Retrying = 4,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                    IsolatedWindowsEnvironmentProgressState_Starting = 5,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                    IsolatedWindowsEnvironmentProgressState_Finalizing = 6,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentShareFileStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentShareFileStatus : int
                {
                    IsolatedWindowsEnvironmentShareFileStatus_Success = 0,
                    IsolatedWindowsEnvironmentShareFileStatus_UnknownFailure = 1,
                    IsolatedWindowsEnvironmentShareFileStatus_EnvironmentUnavailable = 2,
                    IsolatedWindowsEnvironmentShareFileStatus_AlreadySharedWithConflictingOptions = 3,
                    IsolatedWindowsEnvironmentShareFileStatus_FileNotFound = 4,
                    IsolatedWindowsEnvironmentShareFileStatus_AccessDenied = 5,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentShareFolderStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentShareFolderStatus : int
                {
                    IsolatedWindowsEnvironmentShareFolderStatus_Success = 0,
                    IsolatedWindowsEnvironmentShareFolderStatus_UnknownFailure = 1,
                    IsolatedWindowsEnvironmentShareFolderStatus_EnvironmentUnavailable = 2,
                    IsolatedWindowsEnvironmentShareFolderStatus_FolderNotFound = 3,
                    IsolatedWindowsEnvironmentShareFolderStatus_AccessDenied = 4,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentSignInProgress is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentSignInProgress : int
                {
                    IsolatedWindowsEnvironmentSignInProgress_Connecting = 0,
                    IsolatedWindowsEnvironmentSignInProgress_Connected = 1,
                    IsolatedWindowsEnvironmentSignInProgress_Authenticating = 2,
                    IsolatedWindowsEnvironmentSignInProgress_SettingUpAccount = 3,
                    IsolatedWindowsEnvironmentSignInProgress_Finalizing = 4,
                    IsolatedWindowsEnvironmentSignInProgress_Completed = 5,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentStartProcessStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentStartProcessStatus : int
                {
                    IsolatedWindowsEnvironmentStartProcessStatus_Success = 0,
                    IsolatedWindowsEnvironmentStartProcessStatus_UnknownFailure = 1,
                    IsolatedWindowsEnvironmentStartProcessStatus_EnvironmentUnavailable = 2,
                    IsolatedWindowsEnvironmentStartProcessStatus_FileNotFound = 3,
                    IsolatedWindowsEnvironmentStartProcessStatus_AppNotRegistered = 4,
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentCreateProgress is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IsolatedWindowsEnvironmentCreateProgress
                {
                    ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentProgressState State;
                    UINT32 PercentComplete;
                };
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Security.Isolation.HostMessageReceivedCallback
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("faf26ffa-8ce1-4cc1-b278-322d31a5e4a3")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("HostMessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IHostMessageReceivedCallback : public IUnknown
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("HostMessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        GUID receiverId,
                        __FIVectorView_1_IInspectable* message
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IHostMessageReceivedCallback = __uuidof(IHostMessageReceivedCallback);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Delegate Windows.Security.Isolation.MessageReceivedCallback
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_INTERFACE_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("f5b4c8ff-1d9d-4995-9fea-4d15257c0757")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("MessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IMessageReceivedCallback : public IUnknown
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("MessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE Invoke(
                        GUID receiverId,
                        __FIVectorView_1_IInspectable* message
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IMessageReceivedCallback = __uuidof(IMessageReceivedCallback);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("41d24597-c328-4467-b37f-4dfc6f60b6bc")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironment : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE StartProcessSilentlyAsync(
                        HSTRING hostExePath,
                        HSTRING arguments,
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentActivator activator,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE StartProcessSilentlyWithTelemetryAsync(
                        HSTRING hostExePath,
                        HSTRING arguments,
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentActivator activator,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ShareFolderAsync(
                        HSTRING hostFolder,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFolderRequestOptions* requestOptions,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ShareFolderWithTelemetryAsync(
                        HSTRING hostFolder,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFolderRequestOptions* requestOptions,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE LaunchFileWithUIAsync(
                        HSTRING appExePath,
                        HSTRING argumentsTemplate,
                        HSTRING filePath,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE LaunchFileWithUIAndTelemetryAsync(
                        HSTRING appExePath,
                        HSTRING argumentsTemplate,
                        HSTRING filePath,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE TerminateAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE TerminateWithTelemetryAsync(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE RegisterMessageReceiver(
                        GUID receiverId,
                        ABI::Windows::Security::Isolation::IMessageReceivedCallback* messageReceivedCallback
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE UnregisterMessageReceiver(
                        GUID receiverId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironment = __uuidof(IIsolatedWindowsEnvironment);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("2d365f39-88bd-4ab4-93cf-7e2bcef337c0")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironment2 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE PostMessageToReceiverAsync(
                        GUID receiverId,
                        __FIIterable_1_IInspectable* message,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE PostMessageToReceiverWithTelemetryAsync(
                        GUID receiverId,
                        __FIIterable_1_IInspectable* message,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironment2 = __uuidof(IIsolatedWindowsEnvironment2);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment3
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment3[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("cb7fc7d2-d06e-4c26-8ada-dacdaaad03f5")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironment3 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE GetUserInfo(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentUserInfo** result
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ShareFileAsync(
                        HSTRING filePath,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFileRequestOptions* options,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ShareFileWithTelemetryAsync(
                        HSTRING filePath,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentShareFileRequestOptions* options,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironment3 = __uuidof(IIsolatedWindowsEnvironment3);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment4
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment4[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment4";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("11e3701a-dd9e-4f1b-812c-4020f307f93c")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironment4 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ChangePriority(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreationPriority Priority
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironment4 = __uuidof(IIsolatedWindowsEnvironment4);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentCreateResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("ef9a5e58-dcd7-45c2-9c85-ab642a715e8e")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentCreateResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreateStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Environment(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentCreateResult = __uuidof(IIsolatedWindowsEnvironmentCreateResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentCreateResult2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("a547dbc7-61d4-4fb8-ab5c-edefa3d388ad")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentCreateResult2 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ChangeCreationPriority(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreationPriority priority
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentCreateResult2 = __uuidof(IIsolatedWindowsEnvironmentCreateResult2);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentFactory[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("1aca93e7-e804-454d-8466-f9897c20b0f6")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentFactory : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE CreateAsync(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOptions* options,
                        __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE CreateWithTelemetryAsync(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOptions* options,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
                        __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress** operation
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE GetById(
                        HSTRING environmentId,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironment** result
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE FindByOwnerId(
                        HSTRING environmentOwnerId,
                        __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentFactory = __uuidof(IIsolatedWindowsEnvironmentFactory);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentFile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentFile[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("4d5ae1ef-029f-4101-8c35-fe91bf9cd5f0")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentFile : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        GUID* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_HostPath(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE Close(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentFile = __uuidof(IIsolatedWindowsEnvironmentFile);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentFile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentFile2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("4eeb8dec-ad5d-4b0a-b754-f36c3d46d684")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentFile2 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_GuestPath(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_IsReadOnly(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentFile2 = __uuidof(IIsolatedWindowsEnvironmentFile2);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentHost
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentHostStatics[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("2c0e22c7-05a0-517a-b81c-6ee8790c381f")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentHostStatics : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_IsReady(
                        boolean* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_HostErrors(
                        __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentHostStatics = __uuidof(IIsolatedWindowsEnvironmentHostStatics);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentLaunchFileResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("685d4176-f6e0-4569-b1aa-215c0ff5b257")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentLaunchFileResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentLaunchFileStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentFile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentLaunchFileResult = __uuidof(IIsolatedWindowsEnvironmentLaunchFileResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOptions[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("b71d98f7-61f0-4008-b207-0bf9eb2d76f2")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentOptions : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_EnvironmentOwnerId(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_EnvironmentOwnerId(
                        HSTRING value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowedClipboardFormats(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAllowedClipboardFormats* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowedClipboardFormats(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAllowedClipboardFormats value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ClipboardCopyPasteDirections(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentClipboardCopyPasteDirections* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_ClipboardCopyPasteDirections(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentClipboardCopyPasteDirections value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AvailablePrinters(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAvailablePrinters* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AvailablePrinters(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAvailablePrinters value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_SharedHostFolderPath(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_SharedFolderNameInEnvironment(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE ShareHostFolderForUntrustedItems(
                        HSTRING SharedHostFolderPath,
                        HSTRING ShareFolderNameInEnvironment
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_PersistUserProfile(
                        boolean* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_PersistUserProfile(
                        boolean value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowGraphicsHardwareAcceleration(
                        boolean* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowGraphicsHardwareAcceleration(
                        boolean value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowCameraAndMicrophoneAccess(
                        boolean* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowCameraAndMicrophoneAccess(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentOptions = __uuidof(IIsolatedWindowsEnvironmentOptions);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOptions2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("10d7cc31-8b8f-4b9d-b22c-617103b55b08")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentOptions2 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_WindowAnnotationOverride(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_WindowAnnotationOverride(
                        HSTRING value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentOptions2 = __uuidof(IIsolatedWindowsEnvironmentOptions2);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions3
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOptions3[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions3";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("98d5aa23-161f-4cd9-8a9c-269b30122b0d")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentOptions3 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowedClipboardFormatsToEnvironment(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAllowedClipboardFormats* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowedClipboardFormatsToEnvironment(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAllowedClipboardFormats value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowedClipboardFormatsToHost(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAllowedClipboardFormats* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowedClipboardFormatsToHost(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentAllowedClipboardFormats value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_CreationPriority(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreationPriority* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_CreationPriority(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentCreationPriority value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentOptions3 = __uuidof(IIsolatedWindowsEnvironmentOptions3);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOwnerRegistrationData[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("f888ec22-e8cf-56c0-b1df-90af4ad80e84")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentOwnerRegistrationData : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ShareableFolders(
                        __FIVector_1_HSTRING** value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ProcessesRunnableAsSystem(
                        __FIVector_1_HSTRING** value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ProcessesRunnableAsUser(
                        __FIVector_1_HSTRING** value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ActivationFileExtensions(
                        __FIVector_1_HSTRING** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentOwnerRegistrationData = __uuidof(IIsolatedWindowsEnvironmentOwnerRegistrationData);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOwnerRegistrationResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("6dab9451-6169-55df-8f51-790e99d7277d")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentOwnerRegistrationResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentOwnerRegistrationStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentOwnerRegistrationResult = __uuidof(IIsolatedWindowsEnvironmentOwnerRegistrationResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOwnerRegistrationStatics[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("10951754-204b-5ec9-9de3-df792d074a61")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentOwnerRegistrationStatics : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE Register(
                        HSTRING ownerName,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOwnerRegistrationData* ownerRegistrationData,
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentOwnerRegistrationResult** result
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE Unregister(
                        HSTRING ownerName
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentOwnerRegistrationStatics = __uuidof(IIsolatedWindowsEnvironmentOwnerRegistrationStatics);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentPostMessageResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("0dfa28fa-2ef0-4d8f-b341-3171b2df93b1")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentPostMessageResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentPostMessageStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentPostMessageResult = __uuidof(IIsolatedWindowsEnvironmentPostMessageResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentProcess[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("a858c3ef-8172-4f10-af93-cbe60af88d09")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentProcess : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_State(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentProcessState* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExitCode(
                        UINT32* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE WaitForExit(void) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE WaitForExitWithTimeout(
                        UINT32 timeoutMilliseconds
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE WaitForExitAsync(
                        ABI::Windows::Foundation::IAsyncAction** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentProcess = __uuidof(IIsolatedWindowsEnvironmentProcess);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFileRequestOptions[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("c9190ed8-0fd0-4946-bb88-117a60737b61")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentShareFileRequestOptions : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowWrite(
                        boolean* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowWrite(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentShareFileRequestOptions = __uuidof(IIsolatedWindowsEnvironmentShareFileRequestOptions);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFileResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("aec7caa7-9ac6-4bf5-8b91-5c1adf0d7d00")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentShareFileResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFileStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_File(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentFile** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentShareFileResult = __uuidof(IIsolatedWindowsEnvironmentShareFileResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFolderRequestOptions[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("c405eb7d-7053-4f6a-9b87-746846ed19b2")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentShareFolderRequestOptions : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_AllowWrite(
                        boolean* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_AllowWrite(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentShareFolderRequestOptions = __uuidof(IIsolatedWindowsEnvironmentShareFolderRequestOptions);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFolderResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("556ba72e-ca9d-4211-b143-1cedc86eb2fe")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentShareFolderResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentShareFolderStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentShareFolderResult = __uuidof(IIsolatedWindowsEnvironmentShareFolderResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentStartProcessResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("8fa1dc2f-57da-4bb5-9c06-fa072d2032e2")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentStartProcessResult : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Security::Isolation::IsolatedWindowsEnvironmentStartProcessStatus* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_Process(
                        ABI::Windows::Security::Isolation::IIsolatedWindowsEnvironmentProcess** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentStartProcessResult = __uuidof(IIsolatedWindowsEnvironmentStartProcessResult);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentTelemetryParameters[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("ebdb3cab-7a3a-4524-a0f4-f96e284d33cd")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentTelemetryParameters : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_CorrelationId(
                        GUID* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE put_CorrelationId(
                        GUID value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentTelemetryParameters = __uuidof(IIsolatedWindowsEnvironmentTelemetryParameters);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentUserInfo[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("8a9c75ae-69ba-4001-96fc-19a02703b340")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentUserInfo : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_EnvironmentUserSid(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE get_EnvironmentUserName(
                        HSTRING* value
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE TryWaitForSignInAsync(
                        __FIAsyncOperation_1_boolean** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentUserInfo = __uuidof(IIsolatedWindowsEnvironmentUserInfo);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentUserInfo2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("b0bdd5dd-91d7-481e-94f2-2a5a6bdf9383")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsEnvironmentUserInfo2 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE TryWaitForSignInWithProgressAsync(
                        __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress** operation
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsEnvironmentUserInfo2 = __uuidof(IIsolatedWindowsEnvironmentUserInfo2);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsHostMessenger
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsHostMessengerStatics[] = L"Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("06e444bb-53c0-4889-8fa3-53592e37cf21")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsHostMessengerStatics : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE PostMessageToReceiver(
                        GUID receiverId,
                        __FIVectorView_1_IInspectable* message
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE GetFileId(
                        HSTRING filePath,
                        GUID* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsHostMessengerStatics = __uuidof(IIsolatedWindowsHostMessengerStatics);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsHostMessenger
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsHostMessengerStatics2[] = L"Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2";
namespace ABI {
    namespace Windows {
        namespace Security {
            namespace Isolation {
                MIDL_INTERFACE("55ef9ebc-0444-42ad-832d-1b89c089d1ca")
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                IIsolatedWindowsHostMessengerStatics2 : public IInspectable
                {
                public:
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE RegisterHostMessageReceiver(
                        GUID receiverId,
                        ABI::Windows::Security::Isolation::IHostMessageReceivedCallback* hostMessageReceivedCallback
                        ) = 0;
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
                    virtual HRESULT STDMETHODCALLTYPE UnregisterHostMessageReceiver(
                        GUID receiverId
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IIsolatedWindowsHostMessengerStatics2 = __uuidof(IIsolatedWindowsHostMessengerStatics2);
            } /* Isolation */
        } /* Security */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment2
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment3
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironment_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironment_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironment[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironment";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentCreateResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentCreateResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentCreateResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentFile
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentFile_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentFile_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentFile[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentHost
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentHost_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentHost_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentHost[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentLaunchFileResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentLaunchFileResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentLaunchFileResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOptions_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOptions[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistration_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistration_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistration[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationData_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationData_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationData[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentPostMessageResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentPostMessageResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentPostMessageResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentProcess_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentProcess_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentProcess[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileRequestOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileRequestOptions_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileRequestOptions[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderRequestOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderRequestOptions_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderRequestOptions[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentStartProcessResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentStartProcessResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentStartProcessResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentTelemetryParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentTelemetryParameters_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentTelemetryParameters[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentUserInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentUserInfo_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentUserInfo[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsHostMessenger
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2 interface starting with version 2.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsHostMessenger_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsHostMessenger_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsHostMessenger[] = L"Windows.Security.Isolation.IsolatedWindowsHostMessenger";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2 __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2;

#endif // ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult;

typedef struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResultVtbl;

interface __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* This,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentSignInProgress __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentSignInProgress;

typedef interface __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

typedef interface __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

typedef struct __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        boolean* result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl;

interface __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress;

typedef struct __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* This,
        __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress* asyncInfo,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentSignInProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

typedef struct __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateProgress __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateProgress;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress* asyncInfo,
        struct __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if !defined(____FIIterator_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterator_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterator_1_IInspectable __FIIterator_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_IInspectable;

typedef struct __FIIterator_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_IInspectable* This,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_IInspectable* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_IInspectable* This,
        UINT32 itemsLength,
        IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_IInspectableVtbl;

interface __FIIterator_1_IInspectable
{
    CONST_VTBL struct __FIIterator_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_IInspectable_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_IInspectable_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_IInspectable_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_IInspectable_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_IInspectable_INTERFACE_DEFINED__

#if !defined(____FIIterable_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIIterable_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIIterable_1_IInspectable __FIIterable_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_IInspectable;

typedef struct __FIIterable_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_IInspectable* This,
        __FIIterator_1_IInspectable** result);

    END_INTERFACE
} __FIIterable_1_IInspectableVtbl;

interface __FIIterable_1_IInspectable
{
    CONST_VTBL struct __FIIterable_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_IInspectable_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_IInspectable_INTERFACE_DEFINED__

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

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment;

typedef struct __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl;

interface __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment;

typedef struct __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl;

interface __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError;

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError;

typedef struct __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl;

interface __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError
{
    CONST_VTBL struct __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError;

typedef struct __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        __FIIterator_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError** result);

    END_INTERFACE
} __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl;

interface __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError
{
    CONST_VTBL struct __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if !defined(____FIVectorView_1_IInspectable_INTERFACE_DEFINED__)
#define ____FIVectorView_1_IInspectable_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_IInspectable __FIVectorView_1_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_IInspectable;

typedef struct __FIVectorView_1_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_IInspectable* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_IInspectable* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_IInspectable* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_IInspectable* This,
        UINT32 index,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_IInspectable* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_IInspectable* This,
        IInspectable* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_IInspectable* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        IInspectable** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_IInspectableVtbl;

interface __FIVectorView_1_IInspectable
{
    CONST_VTBL struct __FIVectorView_1_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_IInspectable_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_IInspectable_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_IInspectable_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_IInspectable_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_IInspectable_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_IInspectable_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_IInspectable_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_IInspectable_INTERFACE_DEFINED__

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

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment;

typedef struct __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        UINT32 index,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl;

interface __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError;

typedef struct __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        UINT32 index,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl;

interface __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError
{
    CONST_VTBL struct __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostErrorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError_INTERFACE_DEFINED__
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

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

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentActivator __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentActivator;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAvailablePrinters __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAvailablePrinters;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentClipboardCopyPasteDirections __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentClipboardCopyPasteDirections;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateStatus;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentLaunchFileStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentLaunchFileStatus;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentOwnerRegistrationStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentOwnerRegistrationStatus;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentPostMessageStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentPostMessageStatus;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProcessState __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProcessState;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProgressState __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProgressState;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFileStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFileStatus;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFolderStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFolderStatus;

typedef enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentStartProcessStatus __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentStartProcessStatus;

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentActivator
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentActivator is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentActivator
{
    IsolatedWindowsEnvironmentActivator_System = 0,
    IsolatedWindowsEnvironmentActivator_User = 1,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentAllowedClipboardFormats
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentAllowedClipboardFormats is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats
{
    IsolatedWindowsEnvironmentAllowedClipboardFormats_None = 0,
    IsolatedWindowsEnvironmentAllowedClipboardFormats_Text = 0x1,
    IsolatedWindowsEnvironmentAllowedClipboardFormats_Image = 0x2,
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
    IsolatedWindowsEnvironmentAllowedClipboardFormats_Rtf = 0x4,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentAvailablePrinters
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentAvailablePrinters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAvailablePrinters
{
    IsolatedWindowsEnvironmentAvailablePrinters_None = 0,
    IsolatedWindowsEnvironmentAvailablePrinters_Local = 0x1,
    IsolatedWindowsEnvironmentAvailablePrinters_Network = 0x2,
    IsolatedWindowsEnvironmentAvailablePrinters_SystemPrintToPdf = 0x4,
    IsolatedWindowsEnvironmentAvailablePrinters_SystemPrintToXps = 0x8,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentClipboardCopyPasteDirections
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentClipboardCopyPasteDirections is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentClipboardCopyPasteDirections
{
    IsolatedWindowsEnvironmentClipboardCopyPasteDirections_None = 0,
    IsolatedWindowsEnvironmentClipboardCopyPasteDirections_HostToIsolatedWindowsEnvironment = 0x1,
    IsolatedWindowsEnvironmentClipboardCopyPasteDirections_IsolatedWindowsEnvironmentToHost = 0x2,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreateStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateStatus
{
    IsolatedWindowsEnvironmentCreateStatus_Success = 0,
    IsolatedWindowsEnvironmentCreateStatus_FailureByPolicy = 1,
    IsolatedWindowsEnvironmentCreateStatus_UnknownFailure = 2,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentCreationPriority
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreationPriority is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority
{
    IsolatedWindowsEnvironmentCreationPriority_Low = 0,
    IsolatedWindowsEnvironmentCreationPriority_Normal = 1,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentHostError
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentHostError is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentHostError
{
    IsolatedWindowsEnvironmentHostError_AdminPolicyIsDisabledOrNotPresent = 0,
    IsolatedWindowsEnvironmentHostError_FeatureNotInstalled = 1,
    IsolatedWindowsEnvironmentHostError_HardwareRequirementsNotMet = 2,
    IsolatedWindowsEnvironmentHostError_RebootRequired = 3,
    IsolatedWindowsEnvironmentHostError_UnknownError = 4,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentLaunchFileStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentLaunchFileStatus
{
    IsolatedWindowsEnvironmentLaunchFileStatus_Success = 0,
    IsolatedWindowsEnvironmentLaunchFileStatus_UnknownFailure = 1,
    IsolatedWindowsEnvironmentLaunchFileStatus_EnvironmentUnavailable = 2,
    IsolatedWindowsEnvironmentLaunchFileStatus_FileNotFound = 3,
    IsolatedWindowsEnvironmentLaunchFileStatus_TimedOut = 4,
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
    IsolatedWindowsEnvironmentLaunchFileStatus_AlreadySharedWithConflictingOptions = 5,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentOwnerRegistrationStatus
{
    IsolatedWindowsEnvironmentOwnerRegistrationStatus_Success = 0,
    IsolatedWindowsEnvironmentOwnerRegistrationStatus_InvalidArgument = 1,
    IsolatedWindowsEnvironmentOwnerRegistrationStatus_AccessDenied = 2,
    IsolatedWindowsEnvironmentOwnerRegistrationStatus_InsufficientMemory = 3,
    IsolatedWindowsEnvironmentOwnerRegistrationStatus_UnknownFailure = 4,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentPostMessageStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentPostMessageStatus
{
    IsolatedWindowsEnvironmentPostMessageStatus_Success = 0,
    IsolatedWindowsEnvironmentPostMessageStatus_UnknownFailure = 1,
    IsolatedWindowsEnvironmentPostMessageStatus_EnvironmentUnavailable = 2,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentProcessState
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentProcessState is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProcessState
{
    IsolatedWindowsEnvironmentProcessState_Running = 1,
    IsolatedWindowsEnvironmentProcessState_Aborted = 2,
    IsolatedWindowsEnvironmentProcessState_Completed = 3,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentProgressState
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentProgressState is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProgressState
{
    IsolatedWindowsEnvironmentProgressState_Queued = 0,
    IsolatedWindowsEnvironmentProgressState_Processing = 1,
    IsolatedWindowsEnvironmentProgressState_Completed = 2,
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
    IsolatedWindowsEnvironmentProgressState_Creating = 3,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
    IsolatedWindowsEnvironmentProgressState_Retrying = 4,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
    IsolatedWindowsEnvironmentProgressState_Starting = 5,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
    IsolatedWindowsEnvironmentProgressState_Finalizing = 6,
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFileStatus
{
    IsolatedWindowsEnvironmentShareFileStatus_Success = 0,
    IsolatedWindowsEnvironmentShareFileStatus_UnknownFailure = 1,
    IsolatedWindowsEnvironmentShareFileStatus_EnvironmentUnavailable = 2,
    IsolatedWindowsEnvironmentShareFileStatus_AlreadySharedWithConflictingOptions = 3,
    IsolatedWindowsEnvironmentShareFileStatus_FileNotFound = 4,
    IsolatedWindowsEnvironmentShareFileStatus_AccessDenied = 5,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFolderStatus
{
    IsolatedWindowsEnvironmentShareFolderStatus_Success = 0,
    IsolatedWindowsEnvironmentShareFolderStatus_UnknownFailure = 1,
    IsolatedWindowsEnvironmentShareFolderStatus_EnvironmentUnavailable = 2,
    IsolatedWindowsEnvironmentShareFolderStatus_FolderNotFound = 3,
    IsolatedWindowsEnvironmentShareFolderStatus_AccessDenied = 4,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentSignInProgress
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentSignInProgress is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentSignInProgress
{
    IsolatedWindowsEnvironmentSignInProgress_Connecting = 0,
    IsolatedWindowsEnvironmentSignInProgress_Connected = 1,
    IsolatedWindowsEnvironmentSignInProgress_Authenticating = 2,
    IsolatedWindowsEnvironmentSignInProgress_SettingUpAccount = 3,
    IsolatedWindowsEnvironmentSignInProgress_Finalizing = 4,
    IsolatedWindowsEnvironmentSignInProgress_Completed = 5,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessStatus
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
enum
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentStartProcessStatus is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentStartProcessStatus
{
    IsolatedWindowsEnvironmentStartProcessStatus_Success = 0,
    IsolatedWindowsEnvironmentStartProcessStatus_UnknownFailure = 1,
    IsolatedWindowsEnvironmentStartProcessStatus_EnvironmentUnavailable = 2,
    IsolatedWindowsEnvironmentStartProcessStatus_FileNotFound = 3,
    IsolatedWindowsEnvironmentStartProcessStatus_AppNotRegistered = 4,
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateProgress
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreateProgress is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateProgress
{
    enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProgressState State;
    UINT32 PercentComplete;
};
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Delegate Windows.Security.Isolation.HostMessageReceivedCallback
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("HostMessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback* This);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("HostMessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback* This,
        GUID receiverId,
        __FIVectorView_1_IInspectable* message);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallbackVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("HostMessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_Invoke(This, receiverId, message) \
    ((This)->lpVtbl->Invoke(This, receiverId, message))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Delegate Windows.Security.Isolation.MessageReceivedCallback
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_INTERFACE_DEFINED__
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("MessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallbackVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback* This);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("MessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* Invoke)(__x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback* This,
        GUID receiverId,
        __FIVectorView_1_IInspectable* message);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallbackVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallbackVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_Release(This) \
    ((This)->lpVtbl->Release(This))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("MessageReceivedCallback is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_Invoke(This, receiverId, message) \
    ((This)->lpVtbl->Invoke(This, receiverId, message))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* StartProcessSilentlyAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING hostExePath,
        HSTRING arguments,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentActivator activator,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* StartProcessSilentlyWithTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING hostExePath,
        HSTRING arguments,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentActivator activator,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentStartProcessResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ShareFolderAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING hostFolder,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* requestOptions,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ShareFolderWithTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING hostFolder,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* requestOptions,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFolderResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* LaunchFileWithUIAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING appExePath,
        HSTRING argumentsTemplate,
        HSTRING filePath,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* LaunchFileWithUIAndTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        HSTRING appExePath,
        HSTRING argumentsTemplate,
        HSTRING filePath,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentLaunchFileResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* TerminateAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* TerminateWithTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* RegisterMessageReceiver)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        GUID receiverId,
        __x_ABI_CWindows_CSecurity_CIsolation_CIMessageReceivedCallback* messageReceivedCallback);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* UnregisterMessageReceiver)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment* This,
        GUID receiverId);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_StartProcessSilentlyAsync(This, hostExePath, arguments, activator, operation) \
    ((This)->lpVtbl->StartProcessSilentlyAsync(This, hostExePath, arguments, activator, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_StartProcessSilentlyWithTelemetryAsync(This, hostExePath, arguments, activator, telemetryParameters, operation) \
    ((This)->lpVtbl->StartProcessSilentlyWithTelemetryAsync(This, hostExePath, arguments, activator, telemetryParameters, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_ShareFolderAsync(This, hostFolder, requestOptions, operation) \
    ((This)->lpVtbl->ShareFolderAsync(This, hostFolder, requestOptions, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_ShareFolderWithTelemetryAsync(This, hostFolder, requestOptions, telemetryParameters, operation) \
    ((This)->lpVtbl->ShareFolderWithTelemetryAsync(This, hostFolder, requestOptions, telemetryParameters, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_LaunchFileWithUIAsync(This, appExePath, argumentsTemplate, filePath, operation) \
    ((This)->lpVtbl->LaunchFileWithUIAsync(This, appExePath, argumentsTemplate, filePath, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_LaunchFileWithUIAndTelemetryAsync(This, appExePath, argumentsTemplate, filePath, telemetryParameters, operation) \
    ((This)->lpVtbl->LaunchFileWithUIAndTelemetryAsync(This, appExePath, argumentsTemplate, filePath, telemetryParameters, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_TerminateAsync(This, operation) \
    ((This)->lpVtbl->TerminateAsync(This, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_TerminateWithTelemetryAsync(This, telemetryParameters, operation) \
    ((This)->lpVtbl->TerminateWithTelemetryAsync(This, telemetryParameters, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_RegisterMessageReceiver(This, receiverId, messageReceivedCallback) \
    ((This)->lpVtbl->RegisterMessageReceiver(This, receiverId, messageReceivedCallback))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_UnregisterMessageReceiver(This, receiverId) \
    ((This)->lpVtbl->UnregisterMessageReceiver(This, receiverId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment2";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* PostMessageToReceiverAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This,
        GUID receiverId,
        __FIIterable_1_IInspectable* message,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* PostMessageToReceiverWithTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2* This,
        GUID receiverId,
        __FIIterable_1_IInspectable* message,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentPostMessageResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_PostMessageToReceiverAsync(This, receiverId, message, operation) \
    ((This)->lpVtbl->PostMessageToReceiverAsync(This, receiverId, message, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_PostMessageToReceiverWithTelemetryAsync(This, receiverId, message, telemetryParameters, operation) \
    ((This)->lpVtbl->PostMessageToReceiverWithTelemetryAsync(This, receiverId, message, telemetryParameters, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment3
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment3[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment3";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* GetUserInfo)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo** result);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ShareFileAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        HSTRING filePath,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* options,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ShareFileWithTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3* This,
        HSTRING filePath,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* options,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __FIAsyncOperation_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentShareFileResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_GetUserInfo(This, result) \
    ((This)->lpVtbl->GetUserInfo(This, result))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_ShareFileAsync(This, filePath, options, operation) \
    ((This)->lpVtbl->ShareFileAsync(This, filePath, options, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_ShareFileWithTelemetryAsync(This, filePath, options, telemetryParameters, operation) \
    ((This)->lpVtbl->ShareFileWithTelemetryAsync(This, filePath, options, telemetryParameters, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment3_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironment4
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironment4[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironment4";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ChangePriority)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority Priority);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_ChangePriority(This, Priority) \
    ((This)->lpVtbl->ChangePriority(This, Priority))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment4_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentCreateResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreateStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        HRESULT* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Environment)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_get_Environment(This, value) \
    ((This)->lpVtbl->get_Environment(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentCreateResult2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult2";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ChangeCreationPriority)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority priority);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_ChangeCreationPriority(This, priority) \
    ((This)->lpVtbl->ChangeCreationPriority(This, priority))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentCreateResult2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentFactory[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* CreateAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* options,
        __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* CreateWithTelemetryAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* options,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* telemetryParameters,
        __FIAsyncOperationWithProgress_2_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateResult_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentCreateProgress** operation);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* GetById)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        HSTRING environmentId,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironment** result);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* FindByOwnerId)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory* This,
        HSTRING environmentOwnerId,
        __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironment** result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactoryVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_CreateAsync(This, options, operation) \
    ((This)->lpVtbl->CreateAsync(This, options, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_CreateWithTelemetryAsync(This, options, telemetryParameters, operation) \
    ((This)->lpVtbl->CreateWithTelemetryAsync(This, options, telemetryParameters, operation))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_GetById(This, environmentId, result) \
    ((This)->lpVtbl->GetById(This, environmentId, result))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_FindByOwnerId(This, environmentOwnerId, result) \
    ((This)->lpVtbl->FindByOwnerId(This, environmentOwnerId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentFile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentFile[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFileVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This,
        GUID* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_HostPath)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* Close)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile* This);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFileVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFileVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_get_HostPath(This, value) \
    ((This)->lpVtbl->get_HostPath(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_Close(This) \
    ((This)->lpVtbl->Close(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentFile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentFile2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_GuestPath)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_IsReadOnly)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_get_GuestPath(This, value) \
    ((This)->lpVtbl->get_GuestPath(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_get_IsReadOnly(This, value) \
    ((This)->lpVtbl->get_IsReadOnly(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentHost
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentHostStatics[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_IsReady)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This,
        boolean* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_HostErrors)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics* This,
        __FIVectorView_1_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentHostError** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_get_IsReady(This, value) \
    ((This)->lpVtbl->get_IsReady(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_get_HostErrors(This, value) \
    ((This)->lpVtbl->get_HostErrors(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentHostStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentLaunchFileResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentLaunchFileStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        HRESULT* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentLaunchFileResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOptions[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_EnvironmentOwnerId)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_EnvironmentOwnerId)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        HSTRING value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowedClipboardFormats)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowedClipboardFormats)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ClipboardCopyPasteDirections)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentClipboardCopyPasteDirections* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_ClipboardCopyPasteDirections)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentClipboardCopyPasteDirections value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AvailablePrinters)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAvailablePrinters* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AvailablePrinters)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAvailablePrinters value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_SharedHostFolderPath)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_SharedFolderNameInEnvironment)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* ShareHostFolderForUntrustedItems)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        HSTRING SharedHostFolderPath,
        HSTRING ShareFolderNameInEnvironment);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_PersistUserProfile)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        boolean* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_PersistUserProfile)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        boolean value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowGraphicsHardwareAcceleration)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        boolean* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowGraphicsHardwareAcceleration)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        boolean value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowCameraAndMicrophoneAccess)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        boolean* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowCameraAndMicrophoneAccess)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptionsVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_EnvironmentOwnerId(This, value) \
    ((This)->lpVtbl->get_EnvironmentOwnerId(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_EnvironmentOwnerId(This, value) \
    ((This)->lpVtbl->put_EnvironmentOwnerId(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_AllowedClipboardFormats(This, value) \
    ((This)->lpVtbl->get_AllowedClipboardFormats(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_AllowedClipboardFormats(This, value) \
    ((This)->lpVtbl->put_AllowedClipboardFormats(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_ClipboardCopyPasteDirections(This, value) \
    ((This)->lpVtbl->get_ClipboardCopyPasteDirections(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_ClipboardCopyPasteDirections(This, value) \
    ((This)->lpVtbl->put_ClipboardCopyPasteDirections(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_AvailablePrinters(This, value) \
    ((This)->lpVtbl->get_AvailablePrinters(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_AvailablePrinters(This, value) \
    ((This)->lpVtbl->put_AvailablePrinters(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_SharedHostFolderPath(This, value) \
    ((This)->lpVtbl->get_SharedHostFolderPath(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_SharedFolderNameInEnvironment(This, value) \
    ((This)->lpVtbl->get_SharedFolderNameInEnvironment(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_ShareHostFolderForUntrustedItems(This, SharedHostFolderPath, ShareFolderNameInEnvironment) \
    ((This)->lpVtbl->ShareHostFolderForUntrustedItems(This, SharedHostFolderPath, ShareFolderNameInEnvironment))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_PersistUserProfile(This, value) \
    ((This)->lpVtbl->get_PersistUserProfile(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_PersistUserProfile(This, value) \
    ((This)->lpVtbl->put_PersistUserProfile(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_AllowGraphicsHardwareAcceleration(This, value) \
    ((This)->lpVtbl->get_AllowGraphicsHardwareAcceleration(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_AllowGraphicsHardwareAcceleration(This, value) \
    ((This)->lpVtbl->put_AllowGraphicsHardwareAcceleration(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_get_AllowCameraAndMicrophoneAccess(This, value) \
    ((This)->lpVtbl->get_AllowCameraAndMicrophoneAccess(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_put_AllowCameraAndMicrophoneAccess(This, value) \
    ((This)->lpVtbl->put_AllowCameraAndMicrophoneAccess(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOptions2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_WindowAnnotationOverride)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_WindowAnnotationOverride)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2* This,
        HSTRING value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_get_WindowAnnotationOverride(This, value) \
    ((This)->lpVtbl->get_WindowAnnotationOverride(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_put_WindowAnnotationOverride(This, value) \
    ((This)->lpVtbl->put_WindowAnnotationOverride(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions3
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOptions3[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions3";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowedClipboardFormatsToEnvironment)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowedClipboardFormatsToEnvironment)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowedClipboardFormatsToHost)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowedClipboardFormatsToHost)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentAllowedClipboardFormats value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_CreationPriority)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_CreationPriority)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentCreationPriority value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_get_AllowedClipboardFormatsToEnvironment(This, value) \
    ((This)->lpVtbl->get_AllowedClipboardFormatsToEnvironment(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_put_AllowedClipboardFormatsToEnvironment(This, value) \
    ((This)->lpVtbl->put_AllowedClipboardFormatsToEnvironment(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_get_AllowedClipboardFormatsToHost(This, value) \
    ((This)->lpVtbl->get_AllowedClipboardFormatsToHost(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_put_AllowedClipboardFormatsToHost(This, value) \
    ((This)->lpVtbl->put_AllowedClipboardFormatsToHost(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_get_CreationPriority(This, value) \
    ((This)->lpVtbl->get_CreationPriority(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_put_CreationPriority(This, value) \
    ((This)->lpVtbl->put_CreationPriority(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOptions3_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOwnerRegistrationData[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ShareableFolders)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        __FIVector_1_HSTRING** value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ProcessesRunnableAsSystem)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        __FIVector_1_HSTRING** value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ProcessesRunnableAsUser)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        __FIVector_1_HSTRING** value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ActivationFileExtensions)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* This,
        __FIVector_1_HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationDataVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_get_ShareableFolders(This, value) \
    ((This)->lpVtbl->get_ShareableFolders(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_get_ProcessesRunnableAsSystem(This, value) \
    ((This)->lpVtbl->get_ProcessesRunnableAsSystem(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_get_ProcessesRunnableAsUser(This, value) \
    ((This)->lpVtbl->get_ProcessesRunnableAsUser(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_get_ActivationFileExtensions(This, value) \
    ((This)->lpVtbl->get_ActivationFileExtensions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOwnerRegistrationResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentOwnerRegistrationStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentOwnerRegistrationStatics[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* Register)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This,
        HSTRING ownerName,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationData* ownerRegistrationData,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationResult** result);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* Unregister)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics* This,
        HSTRING ownerName);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_Register(This, ownerName, ownerRegistrationData, result) \
    ((This)->lpVtbl->Register(This, ownerName, ownerRegistrationData, result))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_Unregister(This, ownerName) \
    ((This)->lpVtbl->Unregister(This, ownerName))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentOwnerRegistrationStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentPostMessageResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentPostMessageStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentPostMessageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentProcess[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcessVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentProcessState* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExitCode)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        UINT32* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* WaitForExit)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* WaitForExitWithTimeout)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        UINT32 timeoutMilliseconds);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* WaitForExitAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcessVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcessVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_get_ExitCode(This, value) \
    ((This)->lpVtbl->get_ExitCode(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_WaitForExit(This) \
    ((This)->lpVtbl->WaitForExit(This))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_WaitForExitWithTimeout(This, timeoutMilliseconds) \
    ((This)->lpVtbl->WaitForExitWithTimeout(This, timeoutMilliseconds))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_WaitForExitAsync(This, operation) \
    ((This)->lpVtbl->WaitForExitAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFileRequestOptions[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowWrite)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This,
        boolean* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowWrite)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptionsVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_get_AllowWrite(This, value) \
    ((This)->lpVtbl->get_AllowWrite(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_put_AllowWrite(This, value) \
    ((This)->lpVtbl->put_AllowWrite(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileRequestOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFileResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFileStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        HRESULT* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_File)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentFile** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_get_File(This, value) \
    ((This)->lpVtbl->get_File(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFileResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFolderRequestOptions[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_AllowWrite)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This,
        boolean* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_AllowWrite)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptionsVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_get_AllowWrite(This, value) \
    ((This)->lpVtbl->get_AllowWrite(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_put_AllowWrite(This, value) \
    ((This)->lpVtbl->put_AllowWrite(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderRequestOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentShareFolderResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentShareFolderStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentShareFolderResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentStartProcessResult[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        enum __x_ABI_CWindows_CSecurity_CIsolation_CIsolatedWindowsEnvironmentStartProcessStatus* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        HRESULT* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_Process)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult* This,
        __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentProcess** value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResultVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_get_Process(This, value) \
    ((This)->lpVtbl->get_Process(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentStartProcessResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentTelemetryParameters[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParametersVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_CorrelationId)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This,
        GUID* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* put_CorrelationId)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters* This,
        GUID value);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParametersVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParametersVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_get_CorrelationId(This, value) \
    ((This)->lpVtbl->get_CorrelationId(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_put_CorrelationId(This, value) \
    ((This)->lpVtbl->put_CorrelationId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentTelemetryParameters_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentUserInfo[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_EnvironmentUserSid)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* get_EnvironmentUserName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        HSTRING* value);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* TryWaitForSignInAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfoVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_get_EnvironmentUserSid(This, value) \
    ((This)->lpVtbl->get_EnvironmentUserSid(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_get_EnvironmentUserName(This, value) \
    ((This)->lpVtbl->get_EnvironmentUserName(This, value))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_TryWaitForSignInAsync(This, operation) \
    ((This)->lpVtbl->TryWaitForSignInAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsEnvironmentUserInfo2[] = L"Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo2";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* TryWaitForSignInWithProgressAsync)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2* This,
        __FIAsyncOperationWithProgress_2_boolean_Windows__CSecurity__CIsolation__CIsolatedWindowsEnvironmentSignInProgress** operation);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_TryWaitForSignInWithProgressAsync(This, operation) \
    ((This)->lpVtbl->TryWaitForSignInWithProgressAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsEnvironmentUserInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsHostMessenger
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsHostMessengerStatics[] = L"Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* PostMessageToReceiver)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This,
        GUID receiverId,
        __FIVectorView_1_IInspectable* message);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* GetFileId)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics* This,
        HSTRING filePath,
        GUID* result);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStaticsVtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_PostMessageToReceiver(This, receiverId, message) \
    ((This)->lpVtbl->PostMessageToReceiver(This, receiverId, message))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_GetFileId(This, filePath, result) \
    ((This)->lpVtbl->GetFileId(This, filePath, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Security.Isolation.IsolatedWindowsHostMessenger
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Security_Isolation_IIsolatedWindowsHostMessengerStatics2[] = L"Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2";
typedef struct
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This,
        TrustLevel* trustLevel);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* RegisterHostMessageReceiver)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This,
        GUID receiverId,
        __x_ABI_CWindows_CSecurity_CIsolation_CIHostMessageReceivedCallback* hostMessageReceivedCallback);
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    HRESULT (STDMETHODCALLTYPE* UnregisterHostMessageReceiver)(__x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2* This,
        GUID receiverId);

    END_INTERFACE
} __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2Vtbl;

interface __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_RegisterHostMessageReceiver(This, receiverId, hostMessageReceivedCallback) \
    ((This)->lpVtbl->RegisterHostMessageReceiver(This, receiverId, hostMessageReceivedCallback))

#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
    DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
#define __x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_UnregisterHostMessageReceiver(This, receiverId) \
    ((This)->lpVtbl->UnregisterHostMessageReceiver(This, receiverId))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2;
#endif /* !defined(____x_ABI_CWindows_CSecurity_CIsolation_CIIsolatedWindowsHostMessengerStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironment
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsEnvironmentFactory interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment2
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment3
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironment4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironment_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironment_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironment is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironment[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironment";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentCreateResult2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentCreateResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentCreateResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentCreateResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentCreateResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentCreateResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentFile
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentFile2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentFile_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentFile_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentFile is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentFile[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentFile";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentHost
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsEnvironmentHostStatics interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentHost_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentHost_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentHost is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentHost[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentHost";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentLaunchFileResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentLaunchFileResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentLaunchFileResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentLaunchFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentLaunchFileResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentLaunchFileResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions2
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOptions3
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOptions_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOptions[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOptions";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationStatics interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistration_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistration_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistration is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistration[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistration";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationData_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationData_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationData is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationData[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationData";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentOwnerRegistrationResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentOwnerRegistrationResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentOwnerRegistrationResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentOwnerRegistrationResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentPostMessageResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentPostMessageResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentPostMessageResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentPostMessageResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentPostMessageResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentPostMessageResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentProcess ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentProcess_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentProcess_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentProcess is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentProcess[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentProcess";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 3.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileRequestOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileRequestOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileRequestOptions_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileRequestOptions[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileRequestOptions";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFileResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFileResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFileResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFileResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderRequestOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderRequestOptions_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderRequestOptions_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderRequestOptions is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderRequestOptions[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderRequestOptions";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentShareFolderResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentShareFolderResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentShareFolderResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentShareFolderResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentStartProcessResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentStartProcessResult_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentStartProcessResult_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentStartProcessResult is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentStartProcessResult[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentStartProcessResult";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentTelemetryParameters ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentTelemetryParameters_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentTelemetryParameters_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentTelemetryParameters is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentTelemetryParameters[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentTelemetryParameters";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo ** Default Interface **
 *    Windows.Security.Isolation.IIsolatedWindowsEnvironmentUserInfo2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentUserInfo_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsEnvironmentUserInfo_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsEnvironmentUserInfo is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsEnvironmentUserInfo[] = L"Windows.Security.Isolation.IsolatedWindowsEnvironmentUserInfo";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.Security.Isolation.IsolatedWindowsHostMessenger
 *
 * Introduced to Windows.Security.Isolation.IsolatedWindowsEnvironmentContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics2 interface starting with version 2.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *   Static Methods exist on the Windows.Security.Isolation.IIsolatedWindowsHostMessengerStatics interface starting with version 1.0 of the Windows.Security.Isolation.IsolatedWindowsEnvironmentContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsHostMessenger_DEFINED
#define RUNTIMECLASS_Windows_Security_Isolation_IsolatedWindowsHostMessenger_DEFINED
#if WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
DEPRECATED("IsolatedWindowsHostMessenger is deprecated and might not work on all platforms. For more info, see MSDN.")
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x50000
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Security_Isolation_IsolatedWindowsHostMessenger[] = L"Windows.Security.Isolation.IsolatedWindowsHostMessenger";
#endif
#endif // WINDOWS_SECURITY_ISOLATION_ISOLATEDWINDOWSENVIRONMENTCONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Esecurity2Eisolation_p_h__

#endif // __windows2Esecurity2Eisolation_h__
