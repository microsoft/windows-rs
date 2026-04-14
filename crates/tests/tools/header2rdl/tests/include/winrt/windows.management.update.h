
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
#ifndef __windows2Emanagement2Eupdate_h__
#define __windows2Emanagement2Eupdate_h__
#ifndef __windows2Emanagement2Eupdate_p_h__
#define __windows2Emanagement2Eupdate_p_h__


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

#if !defined(WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION)
#define WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION 0x20001
#endif // defined(WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION)

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
#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IPreviewBuildsManager;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager ABI::Windows::Management::Update::IPreviewBuildsManager

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IPreviewBuildsManagerStatics;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics ABI::Windows::Management::Update::IPreviewBuildsManagerStatics

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IPreviewBuildsState;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState ABI::Windows::Management::Update::IPreviewBuildsState

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdate;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate ABI::Windows::Management::Update::IWindowsSoftwareUpdate

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateActionInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateActionInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateActionProgress;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionProgress

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateActionResultInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionResultInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateAppPackageInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateAppPackageInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateAppPackageInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateAppPackageInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateApprovalInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateApprovalInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateApprovalInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateApprovalInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateExecutionInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateExecutionInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateLocalizationInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateLocalizationInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateLocalizationInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateLocalizationInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateOptionalActionInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalActionInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateOptionalActionInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalActionInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateOptionalInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateOptionalInfoFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfoFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProvider;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider ABI::Windows::Management::Update::IWindowsSoftwareUpdateProvider

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProviderActionResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderActionResult

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProviderActionResultFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderActionResultFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProviderFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProviderPayloadFileInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderPayloadFileInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProviderStatus;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderStatus

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateProviderStatusFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderStatusFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateResultFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateResultFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateScanResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult ABI::Windows::Management::Update::IWindowsSoftwareUpdateScanResult

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateScanResultFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateScanResultFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateVersion;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsSoftwareUpdateVersionFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersionFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdate;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate ABI::Windows::Management::Update::IWindowsUpdate

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateActionCompletedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs ABI::Windows::Management::Update::IWindowsUpdateActionCompletedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateActionProgress;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress ABI::Windows::Management::Update::IWindowsUpdateActionProgress

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateActionResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult ABI::Windows::Management::Update::IWindowsUpdateActionResult

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateAdministrator;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator ABI::Windows::Management::Update::IWindowsUpdateAdministrator

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateAdministratorStatics;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics ABI::Windows::Management::Update::IWindowsUpdateAdministratorStatics

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateApprovalData;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData ABI::Windows::Management::Update::IWindowsUpdateApprovalData

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateAttentionRequiredInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo ABI::Windows::Management::Update::IWindowsUpdateAttentionRequiredInfo

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateAttentionRequiredReasonChangedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs ABI::Windows::Management::Update::IWindowsUpdateAttentionRequiredReasonChangedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateGetAdministratorResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult ABI::Windows::Management::Update::IWindowsUpdateGetAdministratorResult

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateItem;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem ABI::Windows::Management::Update::IWindowsUpdateItem

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateManager;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager ABI::Windows::Management::Update::IWindowsUpdateManager

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateManager2;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2 ABI::Windows::Management::Update::IWindowsUpdateManager2

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateManagerFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory ABI::Windows::Management::Update::IWindowsUpdateManagerFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateManagerFactory2;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2 ABI::Windows::Management::Update::IWindowsUpdateManagerFactory2

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateManagerScanOptions;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions ABI::Windows::Management::Update::IWindowsUpdateManagerScanOptions

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateManagerScanOptionsFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory ABI::Windows::Management::Update::IWindowsUpdateManagerScanOptionsFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateProgressChangedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs ABI::Windows::Management::Update::IWindowsUpdateProgressChangedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateRestartRequestOptions;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions ABI::Windows::Management::Update::IWindowsUpdateRestartRequestOptions

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateRestartRequestOptionsFactory;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory ABI::Windows::Management::Update::IWindowsUpdateRestartRequestOptionsFactory

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                interface IWindowsUpdateScanCompletedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs ABI::Windows::Management::Update::IWindowsUpdateScanCompletedEventArgs

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_FWD_DEFINED__

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
        namespace Management {
            namespace Update {
                class WindowsUpdateItem;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#define DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0f903002-8142-5dc4-8078-7929a6ebfba0"))
IIterator<ABI::Windows::Management::Update::WindowsUpdateItem*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateItem*, ABI::Windows::Management::Update::IWindowsUpdateItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Update.WindowsUpdateItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Update::WindowsUpdateItem*> __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t;
#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#define DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("73336bf8-168c-50c4-8cf9-ae7c2d14fd2e"))
IIterable<ABI::Windows::Management::Update::WindowsUpdateItem*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateItem*, ABI::Windows::Management::Update::IWindowsUpdateItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Update.WindowsUpdateItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Update::WindowsUpdateItem*> __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t;
#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#define DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("8faa5cdf-6b97-5d4f-804d-443b4fbbdc53"))
IVectorView<ABI::Windows::Management::Update::WindowsUpdateItem*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateItem*, ABI::Windows::Management::Update::IWindowsUpdateItem*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsUpdateItem>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Update::WindowsUpdateItem*> __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t;
#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6fe7014f-431a-5203-9574-5246fc49d6bb"))
IAsyncOperation<__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsUpdateItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem*> __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("958afbc1-54db-5352-a899-d4fca044a6d4"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsUpdateItem>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdate;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE
#define DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2fe5fd91-092b-5c79-a2bc-dd3a096c3dd1"))
IIterator<ABI::Windows::Management::Update::WindowsSoftwareUpdate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdate*, ABI::Windows::Management::Update::IWindowsSoftwareUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Update.WindowsSoftwareUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Update::WindowsSoftwareUpdate*> __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_t;
#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE
#define DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d0cbaacd-54be-5c60-8181-16889db62d34"))
IIterable<ABI::Windows::Management::Update::WindowsSoftwareUpdate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdate*, ABI::Windows::Management::Update::IWindowsSoftwareUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Update.WindowsSoftwareUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Update::WindowsSoftwareUpdate*> __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_t;
#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateActionType : int WindowsSoftwareUpdateActionType;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE
#define DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("55a95ee5-4d1d-5e8e-8d54-4254aabeb5fe"))
IIterator<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType> : IIterator_impl<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Update.WindowsSoftwareUpdateActionType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType> __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_t;
#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE
#define DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("c7725783-deb5-54c2-b8c8-b3914ed338f1"))
IIterable<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType> : IIterable_impl<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Update.WindowsSoftwareUpdateActionType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType> __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_t;
#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateLocalizationInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE
#define DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("6a80b1ba-f6e3-5762-8efc-c7744d9516a6"))
IIterator<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateLocalizationInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*> __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_t;
#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE
#define DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("4e478bf8-753f-5845-b744-717055593c23"))
IIterable<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateLocalizationInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*> __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_t;
#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateProviderPayloadFileInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE
#define DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0c386f9d-9e41-59ab-a2f6-8077827df678"))
IIterator<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderPayloadFileInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*> __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_t;
#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE
#define DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("56014b75-ed7c-5ed8-a1e1-afd9ec70f7f6"))
IIterable<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderPayloadFileInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*> __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_t;
#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdate;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE
#define DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b3c6c75d-acc8-5450-b138-7d0e4780e1a5"))
IIterator<ABI::Windows::Management::Update::WindowsUpdate*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdate*, ABI::Windows::Management::Update::IWindowsUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.Management.Update.WindowsUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::Management::Update::WindowsUpdate*> __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_t;
#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE
#define DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7adc510b-32d8-5b1d-908a-27ca443a880c"))
IIterable<ABI::Windows::Management::Update::WindowsUpdate*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdate*, ABI::Windows::Management::Update::IWindowsUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.Management.Update.WindowsUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::Management::Update::WindowsUpdate*> __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_t;
#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE
#define DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("e71ba25e-3701-506e-afbb-0ac1b0e7205c"))
IVectorView<ABI::Windows::Management::Update::WindowsSoftwareUpdate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdate*, ABI::Windows::Management::Update::IWindowsSoftwareUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsSoftwareUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Update::WindowsSoftwareUpdate*> __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_t;
#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE
#define DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("a58d8c71-4c3f-5288-a2eb-7ef1aa326b71"))
IVectorView<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType> : IVectorView_impl<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsSoftwareUpdateActionType>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType> __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_t;
#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE
#define DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("3145957d-539f-5f3d-8d5d-bc8a95fc9037"))
IVectorView<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateLocalizationInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Update::WindowsSoftwareUpdateLocalizationInfo*> __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_t;
#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE
#define DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("bdcc8fff-5e1d-5a57-874d-6910054cc43e"))
IVectorView<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderPayloadFileInfo*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderPayloadFileInfo*> __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_t;
#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE
#define DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("1eef9339-6038-5751-b5ae-bd89c17a8741"))
IVectorView<ABI::Windows::Management::Update::WindowsUpdate*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdate*, ABI::Windows::Management::Update::IWindowsUpdate*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.Management.Update.WindowsUpdate>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::Management::Update::WindowsUpdate*> __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_t;
#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000


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



#ifndef DEF___FIReference_1_GUID_USE
#define DEF___FIReference_1_GUID_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("7d50f649-632c-51f9-849a-ee49428933ea"))
IReference<GUID> : IReference_impl<GUID>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Guid>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<GUID> __FIReference_1_GUID_t;
#define __FIReference_1_GUID ABI::Windows::Foundation::__FIReference_1_GUID_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_GUID_USE */



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
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateRestartReason : int WindowsSoftwareUpdateRestartReason;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_USE
#define DEF___FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("10a8f99d-91a9-54fc-8757-1af2de2d62b5"))
IReference<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateRestartReason> : IReference_impl<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateRestartReason>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IReference`1<Windows.Management.Update.WindowsSoftwareUpdateRestartReason>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IReference<enum ABI::Windows::Management::Update::WindowsSoftwareUpdateRestartReason> __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_t;
#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason ABI::Windows::Foundation::__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateProviderStatus;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("94ca7cf1-04a6-5f11-81cf-5402d7712713"))
ITypedEventHandler<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderStatus*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderStatus*, ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderStatus*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Update.WindowsSoftwareUpdateProviderStatus, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderStatus*, IInspectable*> __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateManager;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b95a0a5d-28f1-50f5-914a-b55a1a84dcd8"))
ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, IInspectable*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::IWindowsUpdateManager*>, IInspectable*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Update.WindowsUpdateManager, Object>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, IInspectable*> __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_t;
#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateActionCompletedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a4f8bb0a-4633-5c2c-b4a2-5047e201cf07"))
ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateActionCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::IWindowsUpdateManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateActionCompletedEventArgs*, ABI::Windows::Management::Update::IWindowsUpdateActionCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Update.WindowsUpdateManager, Windows.Management.Update.WindowsUpdateActionCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateActionCompletedEventArgs*> __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateAttentionRequiredReasonChangedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1b44e3b5-e822-5c75-85a9-4a3983bebdd8"))
ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateAttentionRequiredReasonChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::IWindowsUpdateManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateAttentionRequiredReasonChangedEventArgs*, ABI::Windows::Management::Update::IWindowsUpdateAttentionRequiredReasonChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Update.WindowsUpdateManager, Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateAttentionRequiredReasonChangedEventArgs*> __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateProgressChangedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("61f35a11-50a1-5199-ac17-75206872fc4d"))
ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateProgressChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::IWindowsUpdateManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateProgressChangedEventArgs*, ABI::Windows::Management::Update::IWindowsUpdateProgressChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Update.WindowsUpdateManager, Windows.Management.Update.WindowsUpdateProgressChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateProgressChangedEventArgs*> __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateScanCompletedEventArgs;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("74625656-399b-5686-b24d-349f57880690"))
ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateScanCompletedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::IWindowsUpdateManager*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::Management::Update::WindowsUpdateScanCompletedEventArgs*, ABI::Windows::Management::Update::IWindowsUpdateScanCompletedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.Management.Update.WindowsUpdateManager, Windows.Management.Update.WindowsUpdateScanCompletedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::Management::Update::WindowsUpdateManager*, ABI::Windows::Management::Update::WindowsUpdateScanCompletedEventArgs*> __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_USE */

#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace Foundation {
            namespace Collections {
                class PropertySet;
            } /* Collections */
        } /* Foundation */
    } /* Windows */
} /* ABI */

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

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateActionResult : int WindowsSoftwareUpdateActionResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateArchitecture : int WindowsSoftwareUpdateArchitecture;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateInstallationType : int WindowsSoftwareUpdateInstallationType;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateProviderRegistrationType : int WindowsSoftwareUpdateProviderRegistrationType;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateProviderTrustState : int WindowsSoftwareUpdateProviderTrustState;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsSoftwareUpdateProviderType : int WindowsSoftwareUpdateProviderType;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsUpdateAdministratorOptions : unsigned int WindowsUpdateAdministratorOptions;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsUpdateAdministratorStatus : int WindowsUpdateAdministratorStatus;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                typedef enum WindowsUpdateAttentionRequiredReason : int WindowsUpdateAttentionRequiredReason;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class PreviewBuildsManager;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class PreviewBuildsState;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateActionInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateActionProgress;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateActionResultInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateAppPackageInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateApprovalInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateExecutionInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateOptionalActionInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateOptionalInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateProvider;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateProviderActionResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateScanResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsSoftwareUpdateVersion;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateActionProgress;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateActionResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateAdministrator;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateApprovalData;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateAttentionRequiredInfo;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateGetAdministratorResult;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateManagerScanOptions;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                class WindowsUpdateRestartRequestOptions;
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateActionResult : int
                {
                    WindowsSoftwareUpdateActionResult_Succeeded = 0,
                    WindowsSoftwareUpdateActionResult_Continue = 1,
                    WindowsSoftwareUpdateActionResult_Failed = 2,
                    WindowsSoftwareUpdateActionResult_Canceled = 3,
                    WindowsSoftwareUpdateActionResult_Removed = 4,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateActionType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateActionType : int
                {
                    WindowsSoftwareUpdateActionType_Download = 0,
                    WindowsSoftwareUpdateActionType_Install = 1,
                    WindowsSoftwareUpdateActionType_Deploy = 2,
                    WindowsSoftwareUpdateActionType_Reboot = 3,
                    WindowsSoftwareUpdateActionType_AppRestart = 4,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateArchitecture
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateArchitecture : int
                {
                    WindowsSoftwareUpdateArchitecture_Neutral = 0,
                    WindowsSoftwareUpdateArchitecture_X86 = 1,
                    WindowsSoftwareUpdateArchitecture_X64 = 2,
                    WindowsSoftwareUpdateArchitecture_Arm = 3,
                    WindowsSoftwareUpdateArchitecture_Arm64 = 4,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateInstallationType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateInstallationType : int
                {
                    WindowsSoftwareUpdateInstallationType_WindowsUpdate = 0,
                    WindowsSoftwareUpdateInstallationType_AppPackage = 1,
                    WindowsSoftwareUpdateInstallationType_Executable = 2,
                    WindowsSoftwareUpdateInstallationType_Powershell = 3,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateProviderRegistrationType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateProviderRegistrationType : int
                {
                    WindowsSoftwareUpdateProviderRegistrationType_None = 0,
                    WindowsSoftwareUpdateProviderRegistrationType_System = 1,
                    WindowsSoftwareUpdateProviderRegistrationType_Windows = 2,
                    WindowsSoftwareUpdateProviderRegistrationType_Pending = 3,
                    WindowsSoftwareUpdateProviderRegistrationType_Registered = 4,
                    WindowsSoftwareUpdateProviderRegistrationType_Unregistered = 5,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateProviderTrustState
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateProviderTrustState : int
                {
                    WindowsSoftwareUpdateProviderTrustState_SignedTrusted = 0,
                    WindowsSoftwareUpdateProviderTrustState_SignedUntrusted = 1,
                    WindowsSoftwareUpdateProviderTrustState_Unsigned = 2,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateProviderType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateProviderType : int
                {
                    WindowsSoftwareUpdateProviderType_WindowsUpdate = 0,
                    WindowsSoftwareUpdateProviderType_Executable = 1,
                    WindowsSoftwareUpdateProviderType_Powershell = 2,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateRestartReason
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsSoftwareUpdateRestartReason : int
                {
                    WindowsSoftwareUpdateRestartReason_None = 0,
                    WindowsSoftwareUpdateRestartReason_System = 1,
                    WindowsSoftwareUpdateRestartReason_AppClose = 2,
                    WindowsSoftwareUpdateRestartReason_AppRestart = 3,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsUpdateAdministratorOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsUpdateAdministratorOptions : unsigned int
                {
                    WindowsUpdateAdministratorOptions_None = 0,
                    WindowsUpdateAdministratorOptions_RequireAdministratorApprovalForScans = 0x1,
                    WindowsUpdateAdministratorOptions_RequireAdministratorApprovalForUpdates = 0x2,
                    WindowsUpdateAdministratorOptions_RequireAdministratorApprovalForActions = 0x4,
                };

                DEFINE_ENUM_FLAG_OPERATORS(WindowsUpdateAdministratorOptions)
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Management.Update.WindowsUpdateAdministratorStatus
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsUpdateAdministratorStatus : int
                {
                    WindowsUpdateAdministratorStatus_Succeeded = 0,
                    WindowsUpdateAdministratorStatus_NoAdministratorRegistered = 1,
                    WindowsUpdateAdministratorStatus_OtherAdministratorIsRegistered = 2,
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Management.Update.WindowsUpdateAttentionRequiredReason
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                enum WindowsUpdateAttentionRequiredReason : int
                {
                    WindowsUpdateAttentionRequiredReason_None = 0,
                    WindowsUpdateAttentionRequiredReason_SeekerUpdate = 1,
                    WindowsUpdateAttentionRequiredReason_ReadyToReboot = 2,
                    WindowsUpdateAttentionRequiredReason_NeedNonMeteredNetwork = 3,
                    WindowsUpdateAttentionRequiredReason_NeedUserAgreementForMeteredNetwork = 4,
                    WindowsUpdateAttentionRequiredReason_NeedNetwork = 5,
                    WindowsUpdateAttentionRequiredReason_NeedMoreSpace = 6,
                    WindowsUpdateAttentionRequiredReason_BatterySaverEnabled = 7,
                    WindowsUpdateAttentionRequiredReason_NeedUserInteraction = 8,
                    WindowsUpdateAttentionRequiredReason_NeedUserAgreementForPolicy = 9,
                    WindowsUpdateAttentionRequiredReason_CompatibilityError = 10,
                    WindowsUpdateAttentionRequiredReason_NeedUserInteractionForEula = 11,
                    WindowsUpdateAttentionRequiredReason_NeedUserInteractionForCta = 12,
                    WindowsUpdateAttentionRequiredReason_Regulated = 13,
                    WindowsUpdateAttentionRequiredReason_ExternalReboot = 14,
                    WindowsUpdateAttentionRequiredReason_OtherUpdate = 15,
                    WindowsUpdateAttentionRequiredReason_BlockedByProvider = 16,
                    WindowsUpdateAttentionRequiredReason_BlockedByPostRebootFailure = 17,
                    WindowsUpdateAttentionRequiredReason_UserEngaged = 18,
                    WindowsUpdateAttentionRequiredReason_BlockedByBattery = 19,
                    WindowsUpdateAttentionRequiredReason_Exclusivity = 20,
                    WindowsUpdateAttentionRequiredReason_BlockedBySerialization = 21,
                    WindowsUpdateAttentionRequiredReason_ConflictClass = 22,
                    WindowsUpdateAttentionRequiredReason_BlockedByAdminApproval = 23,
                    WindowsUpdateAttentionRequiredReason_BlockedByTooManyAttempts = 24,
                    WindowsUpdateAttentionRequiredReason_BlockedByFailure = 25,
                    WindowsUpdateAttentionRequiredReason_Demotion = 26,
                    WindowsUpdateAttentionRequiredReason_BlockedByActiveHours = 27,
                    WindowsUpdateAttentionRequiredReason_ScheduledForMaintenance = 28,
                    WindowsUpdateAttentionRequiredReason_PolicyScheduledInstallTime = 29,
                    WindowsUpdateAttentionRequiredReason_BlockedByOobe = 30,
                    WindowsUpdateAttentionRequiredReason_DeferredDuringOobe = 31,
                    WindowsUpdateAttentionRequiredReason_DeferredForSustainableTime = 32,
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
                    WindowsUpdateAttentionRequiredReason_BlockedByAppClose = 33,
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
                    WindowsUpdateAttentionRequiredReason_BlockedByAppRestart = 34,
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20001
                    WindowsUpdateAttentionRequiredReason_OtherUpdateReverting = 35,
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20001
                };
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IPreviewBuildsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.PreviewBuildsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IPreviewBuildsManager[] = L"Windows.Management.Update.IPreviewBuildsManager";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("fa07dd61-7e4f-59f7-7c9f-def9051c5f62")
                IPreviewBuildsManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ArePreviewBuildsAllowed(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ArePreviewBuildsAllowed(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetCurrentState(
                        ABI::Windows::Management::Update::IPreviewBuildsState** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SyncAsync(
                        __FIAsyncOperation_1_boolean** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPreviewBuildsManager = __uuidof(IPreviewBuildsManager);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Management.Update.IPreviewBuildsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.PreviewBuildsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IPreviewBuildsManagerStatics[] = L"Windows.Management.Update.IPreviewBuildsManagerStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("3e422887-b112-5a70-7da1-97d78d32aa29")
                IPreviewBuildsManagerStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetDefault(
                        ABI::Windows::Management::Update::IPreviewBuildsManager** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE IsSupported(
                        boolean* result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPreviewBuildsManagerStatics = __uuidof(IPreviewBuildsManagerStatics);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Management.Update.IPreviewBuildsState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.PreviewBuildsState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IPreviewBuildsState[] = L"Windows.Management.Update.IPreviewBuildsState";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("a2f2903e-b223-5f63-7546-3e8eac070a2e")
                IPreviewBuildsState : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IPreviewBuildsState = __uuidof(IPreviewBuildsState);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdate
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdate[] = L"Windows.Management.Update.IWindowsSoftwareUpdate";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("d8f19211-98fe-58dd-af0f-470532aa3341")
                IWindowsSoftwareUpdate : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_InstallationType(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateInstallationType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MoreInfoUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DownloadSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstallSizeInBytes(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SourceVersion(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TargetVersion(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProductCode(
                        __FIReference_1_GUID** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Approve(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateApprovalInfo* approvalInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ApproveCurrentAction(
                        boolean approve,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentAction(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionResultInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionResultInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApprovalInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateApprovalInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ApprovedActions(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttentionRequiredInfo(
                        ABI::Windows::Management::Update::IWindowsUpdateAttentionRequiredInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionProgress(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionProgress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RestartReason(
                        __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppPackageInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateAppPackageInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExecutionInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OptionalInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdate = __uuidof(IWindowsSoftwareUpdate);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("2f6723b5-f704-5362-b600-d18808f3973e")
                IWindowsSoftwareUpdateActionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_FileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileArguments(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionType(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateActionInfo = __uuidof(IWindowsSoftwareUpdateActionInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("5e83b58e-d982-5d93-a7cb-bf6c9b6ee5a6")
                IWindowsSoftwareUpdateActionInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING fileName,
                        HSTRING fileArguments,
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateActionType actionType,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateActionInfoFactory = __uuidof(IWindowsSoftwareUpdateActionInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionProgress
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionProgress[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionProgress";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("17dc15fd-75f2-522b-b555-359da8de5581")
                IWindowsSoftwareUpdateActionProgress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentProgress(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TotalProgress(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateActionProgress = __uuidof(IWindowsSoftwareUpdateActionProgress);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionResultInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionResultInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionResultInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("bcf26fba-98c8-51cb-8b7e-1eedc3d82a69")
                IWindowsSoftwareUpdateActionResultInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResultCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateActionResultInfo = __uuidof(IWindowsSoftwareUpdateActionResultInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateAppPackageInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("a5bd59f4-e624-5552-82f9-267a4574a203")
                IWindowsSoftwareUpdateAppPackageInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PackageArchitecture(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateArchitecture* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstallUri(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateAppPackageInfo = __uuidof(IWindowsSoftwareUpdateAppPackageInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateAppPackageInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("a8bda639-a4f6-5a4a-8a54-26c1c508cd0f")
                IWindowsSoftwareUpdateAppPackageInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING packageFamilyName,
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateArchitecture packageArchitecture,
                        ABI::Windows::Foundation::IUriRuntimeClass* installUri,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateAppPackageInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateAppPackageInfoFactory = __uuidof(IWindowsSoftwareUpdateAppPackageInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateApprovalInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("691e6b9e-80af-5882-9404-25437ecb791b")
                IWindowsSoftwareUpdateApprovalInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_UserInitiated(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AppClosure(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MeteredNetwork(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Seeker(
                        boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateApprovalInfo = __uuidof(IWindowsSoftwareUpdateApprovalInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateApprovalInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("ab291c7c-d29f-5ac5-b447-0bfcabdc2cc3")
                IWindowsSoftwareUpdateApprovalInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        boolean userInitiated,
                        boolean appClosure,
                        boolean meteredNetwork,
                        boolean seeker,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateApprovalInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateApprovalInfoFactory = __uuidof(IWindowsSoftwareUpdateApprovalInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateExecutionInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("091aea19-9128-5f24-afc1-a62252df55c0")
                IWindowsSoftwareUpdateExecutionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_DownloadInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_InstallInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_DeployInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OptionalActionInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalActionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateExecutionInfo = __uuidof(IWindowsSoftwareUpdateExecutionInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateExecutionInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("88596f7e-b9ef-5583-8135-94d62ed66ed4")
                IWindowsSoftwareUpdateExecutionInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo* downloadInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo* installInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalActionInfo* actions,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance2(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo* deployInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalActionInfo* actions,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateExecutionInfoFactory = __uuidof(IWindowsSoftwareUpdateExecutionInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdate
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("28e7e01b-4225-52c8-bb51-c68f0b071be5")
                IWindowsSoftwareUpdateFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING providerId,
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateInstallationType installationType,
                        HSTRING updateId,
                        HSTRING title,
                        HSTRING description,
                        ABI::Windows::Foundation::IUriRuntimeClass* moreInfoUrl,
                        UINT64 downloadSizeInBytes,
                        UINT64 installSizeInBytes,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion* sourceVersion,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion* targetVersion,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateAppPackageInfo* appPackageInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfo* executionInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfo* optionalInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance2(
                        HSTRING providerId,
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateInstallationType installationType,
                        HSTRING updateId,
                        HSTRING title,
                        HSTRING description,
                        ABI::Windows::Foundation::IUriRuntimeClass* moreInfoUrl,
                        UINT64 downloadSizeInBytes,
                        UINT64 installSizeInBytes,
                        __FIReference_1_GUID* productCode,
                        HSTRING packageFamilyName,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion* sourceVersion,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion* targetVersion,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateAppPackageInfo* appPackageInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateExecutionInfo* executionInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfo* optionalInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateFactory = __uuidof(IWindowsSoftwareUpdateFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateLocalizationInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("adc2de4b-5966-5f9f-ae07-00d4a285d933")
                IWindowsSoftwareUpdateLocalizationInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LanguageId(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MoreInfoUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateLocalizationInfo = __uuidof(IWindowsSoftwareUpdateLocalizationInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateLocalizationInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("76979b24-f5bd-5c8c-bdb7-a46187374aff")
                IWindowsSoftwareUpdateLocalizationInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        UINT32 languageId,
                        HSTRING title,
                        HSTRING description,
                        ABI::Windows::Foundation::IUriRuntimeClass* moreInfoUrl,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateLocalizationInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateLocalizationInfoFactory = __uuidof(IWindowsSoftwareUpdateLocalizationInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalActionInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("4ac035d0-e50e-5ccb-bfd8-a303562891d2")
                IWindowsSoftwareUpdateOptionalActionInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_CloseAndDeployInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CloseAndInstallInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CloseAndRestartInfo(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateOptionalActionInfo = __uuidof(IWindowsSoftwareUpdateOptionalActionInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalActionInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("88d2fcc1-4791-51b6-b988-966ef93a180b")
                IWindowsSoftwareUpdateOptionalActionInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo* closeAndDeployInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo* closeAndInstallInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateActionInfo* closeAndRestartInfo,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalActionInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateOptionalActionInfoFactory = __uuidof(IWindowsSoftwareUpdateOptionalActionInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("78084a73-50c4-5c33-a751-7a121f5aae70")
                IWindowsSoftwareUpdateOptionalInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_LocalizationInfo(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ComplianceDeadlineInDays(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ComplianceGracePeriodInDays(
                        __FIReference_1_int** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateOptionalInfo = __uuidof(IWindowsSoftwareUpdateOptionalInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfoFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("d837deed-a5f2-5c89-8beb-852d2897b2ef")
                IWindowsSoftwareUpdateOptionalInfoFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        __FIReference_1_int* complianceDeadlineInDays,
                        __FIReference_1_int* complianceGracePeriodInDays,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance2(
                        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* localizationInfo,
                        __FIReference_1_int* complianceDeadlineInDays,
                        __FIReference_1_int* complianceGracePeriodInDays,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateOptionalInfo** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateOptionalInfoFactory = __uuidof(IWindowsSoftwareUpdateOptionalInfoFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProvider
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProvider
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProvider[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProvider";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("20b67f4a-e28e-5d20-9c00-bf249922efbe")
                IWindowsSoftwareUpdateProvider : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE Register(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Unregister(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE Validate(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Id(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Version(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FolderPath(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CatalogFile(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScanFileName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ScanFileArguments(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Type(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PayloadFiles(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrustState(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderTrustState* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RegistrationType(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderRegistrationType* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Properties(
                        ABI::Windows::Foundation::Collections::IPropertySet** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPropertyValue(
                        HSTRING name,
                        IInspectable** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProvider = __uuidof(IWindowsSoftwareUpdateProvider);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderActionResult[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResult";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("afd92b50-6bb9-54de-bdda-9dfb6cc17c16")
                IWindowsSoftwareUpdateProviderActionResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Result(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateActionResult* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RestartReason(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateRestartReason* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResultCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProviderActionResult = __uuidof(IWindowsSoftwareUpdateProviderActionResult);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResultFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderActionResultFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResultFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("0c002684-30c9-59e9-b53f-8846a85d2dc6")
                IWindowsSoftwareUpdateProviderActionResultFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateActionResult actionResult,
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateRestartReason restartReason,
                        UINT32 resultCode,
                        UINT64 extendedError,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderActionResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProviderActionResultFactory = __uuidof(IWindowsSoftwareUpdateProviderActionResultFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProvider
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("fc0d5fc4-e15e-5116-b2ed-db0a64997ffa")
                IWindowsSoftwareUpdateProviderFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING folderPath,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateProvider** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProviderFactory = __uuidof(IWindowsSoftwareUpdateProviderFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderPayloadFileInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderPayloadFileInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderPayloadFileInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("f1da16da-1b01-5367-b4ae-20db8cae1e9b")
                IWindowsSoftwareUpdateProviderPayloadFileInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Filename(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_FileHash(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CatalogFile(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_TrustState(
                        ABI::Windows::Management::Update::WindowsSoftwareUpdateProviderTrustState* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProviderPayloadFileInfo = __uuidof(IWindowsSoftwareUpdateProviderPayloadFileInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderStatus
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderStatus
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderStatus[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderStatus";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("076741b8-7a8e-53b6-9fb7-e290b13c52e9")
                IWindowsSoftwareUpdateProviderStatus : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_CancelRequested(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_CancelRequested(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetScanResult(
                        boolean succeeded,
                        UINT32 resultCode,
                        UINT64 extendedError,
                        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* updates,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetActionProgress(
                        UINT64 current,
                        UINT64 total,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE SetActionResult(
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderActionResult* actionResult,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProviderStatus = __uuidof(IWindowsSoftwareUpdateProviderStatus);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderStatusFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderStatus
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderStatusFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderStatusFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("d1e1b416-7dfd-55ef-9e3c-18d1459e3123")
                IWindowsSoftwareUpdateProviderStatusFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING providerId,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateProviderStatus** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateProviderStatusFactory = __uuidof(IWindowsSoftwareUpdateProviderStatusFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateResult[] = L"Windows.Management.Update.IWindowsSoftwareUpdateResult";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("a6d7ed98-6212-5ad3-aa9d-15e83bb85ee4")
                IWindowsSoftwareUpdateResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CancelRequested(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResultCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        UINT64* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateResult = __uuidof(IWindowsSoftwareUpdateResult);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateResultFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateResultFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateResultFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("512ce0bf-9977-5301-9b29-9e5042c8cf7d")
                IWindowsSoftwareUpdateResultFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        boolean succeeded,
                        UINT32 resultCode,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance2(
                        boolean succeeded,
                        UINT32 resultCode,
                        UINT64 extendedError,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance3(
                        boolean succeeded,
                        boolean cancelRequested,
                        UINT32 resultCode,
                        UINT64 extendedError,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateResultFactory = __uuidof(IWindowsSoftwareUpdateResultFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateScanResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateScanResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateScanResult[] = L"Windows.Management.Update.IWindowsSoftwareUpdateScanResult";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("43ca6d96-3e6d-56da-a903-65d4ab729299")
                IWindowsSoftwareUpdateScanResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ResultCode(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        UINT64* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Updates(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateScanResult = __uuidof(IWindowsSoftwareUpdateScanResult);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateScanResultFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateScanResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateScanResultFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateScanResultFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("21148e4c-e7ce-574e-bfa7-69dc77457d21")
                IWindowsSoftwareUpdateScanResultFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        boolean succeeded,
                        UINT32 resultCode,
                        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* updates,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateScanResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance2(
                        boolean succeeded,
                        UINT32 resultCode,
                        UINT64 extendedError,
                        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* updates,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateScanResult** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateScanResultFactory = __uuidof(IWindowsSoftwareUpdateScanResultFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateVersion
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateVersion
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateVersion[] = L"Windows.Management.Update.IWindowsSoftwareUpdateVersion";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("215e22e7-6d57-5305-9c79-4ecd4a4d7871")
                IWindowsSoftwareUpdateVersion : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Major(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Minor(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RevisionMajor(
                        UINT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_RevisionMinor(
                        UINT32* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateVersion = __uuidof(IWindowsSoftwareUpdateVersion);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateVersionFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateVersion
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateVersionFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateVersionFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("650ed994-0858-5528-a1f2-f73ca64dabc9")
                IWindowsSoftwareUpdateVersionFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        UINT32 major,
                        UINT32 minor,
                        UINT32 revisionMajor,
                        UINT32 revisionMinor,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateVersion** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsSoftwareUpdateVersionFactory = __uuidof(IWindowsSoftwareUpdateVersionFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdate
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdate[] = L"Windows.Management.Update.IWindowsUpdate";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("c3c88dd7-0ef3-52b2-a9ad-66bfc6bd9582")
                IWindowsUpdate : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsFeatureUpdate(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMinorImpact(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSecurity(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsCritical(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsForOS(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsDriver(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsMandatory(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsUrgent(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsSeeker(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MoreInfoUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_SupportUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsEulaAccepted(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_EulaText(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AttentionRequiredInfo(
                        ABI::Windows::Management::Update::IWindowsUpdateAttentionRequiredInfo** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionResult(
                        ABI::Windows::Management::Update::IWindowsUpdateActionResult** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_CurrentAction(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionProgress(
                        ABI::Windows::Management::Update::IWindowsUpdateActionProgress** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetPropertyValue(
                        HSTRING propertyName,
                        IInspectable** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE AcceptEula(void) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdate = __uuidof(IWindowsUpdate);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateActionCompletedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateActionCompletedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("2c44b950-a655-5321-aec1-aee762922131")
                IWindowsUpdateActionCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Update(
                        ABI::Windows::Management::Update::IWindowsUpdate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateActionCompletedEventArgs = __uuidof(IWindowsUpdateActionCompletedEventArgs);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateActionProgress
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateActionProgress[] = L"Windows.Management.Update.IWindowsUpdateActionProgress";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("83b22d8a-4bb0-549f-ba39-59724882d137")
                IWindowsUpdateActionProgress : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Progress(
                        DOUBLE* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateActionProgress = __uuidof(IWindowsUpdateActionProgress);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateActionResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateActionResult[] = L"Windows.Management.Update.IWindowsUpdateActionResult";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("e6692c62-f697-51b7-ab7f-e73e5e688f12")
                IWindowsUpdateActionResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Action(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateActionResult = __uuidof(IWindowsUpdateActionResult);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAdministrator
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAdministrator
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAdministrator[] = L"Windows.Management.Update.IWindowsUpdateAdministrator";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("7a60181c-ba1e-5cf9-aa65-304120b73d72")
                IWindowsUpdateAdministrator : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE StartAdministratorScan(void) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ApproveWindowsUpdateAction(
                        HSTRING updateId,
                        HSTRING action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RevokeWindowsUpdateActionApproval(
                        HSTRING updateId,
                        HSTRING action
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE ApproveWindowsUpdate(
                        HSTRING updateId,
                        ABI::Windows::Management::Update::IWindowsUpdateApprovalData* approvalData
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RevokeWindowsUpdateApproval(
                        HSTRING updateId
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetUpdates(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateAdministrator = __uuidof(IWindowsUpdateAdministrator);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAdministratorStatics
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAdministrator
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAdministratorStatics[] = L"Windows.Management.Update.IWindowsUpdateAdministratorStatics";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("013e6d36-ef69-53bc-8db8-c403bca550ed")
                IWindowsUpdateAdministratorStatics : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetRegisteredAdministrator(
                        HSTRING organizationName,
                        ABI::Windows::Management::Update::IWindowsUpdateGetAdministratorResult** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RegisterForAdministration(
                        HSTRING organizationName,
                        ABI::Windows::Management::Update::WindowsUpdateAdministratorOptions options,
                        ABI::Windows::Management::Update::WindowsUpdateAdministratorStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE UnregisterForAdministration(
                        HSTRING organizationName,
                        ABI::Windows::Management::Update::WindowsUpdateAdministratorStatus* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetRegisteredAdministratorName(
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE RequestRestart(
                        ABI::Windows::Management::Update::IWindowsUpdateRestartRequestOptions* restartOptions,
                        HSTRING* result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE CancelRestartRequest(
                        HSTRING requestRestartToken
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateAdministratorStatics = __uuidof(IWindowsUpdateAdministratorStatics);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateApprovalData
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateApprovalData
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateApprovalData[] = L"Windows.Management.Update.IWindowsUpdateApprovalData";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("aadf5bfd-84db-59bc-85e2-ad4fc1f62f7c")
                IWindowsUpdateApprovalData : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Seeker(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Seeker(
                        __FIReference_1_boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowDownloadOnMetered(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowDownloadOnMetered(
                        __FIReference_1_boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ComplianceDeadlineInDays(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ComplianceDeadlineInDays(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ComplianceGracePeriodInDays(
                        __FIReference_1_int** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ComplianceGracePeriodInDays(
                        __FIReference_1_int* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OptOutOfAutoReboot(
                        __FIReference_1_boolean** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OptOutOfAutoReboot(
                        __FIReference_1_boolean* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateApprovalData = __uuidof(IWindowsUpdateApprovalData);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAttentionRequiredInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAttentionRequiredInfo[] = L"Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("44df2579-74d3-5ffa-b6ce-09e187e1e0ed")
                IWindowsUpdateAttentionRequiredInfo : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        ABI::Windows::Management::Update::WindowsUpdateAttentionRequiredReason* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateAttentionRequiredInfo = __uuidof(IWindowsUpdateAttentionRequiredInfo);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAttentionRequiredReasonChangedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("0627abca-dbb8-524a-b1d2-d9df004eeb31")
                IWindowsUpdateAttentionRequiredReasonChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Update(
                        ABI::Windows::Management::Update::IWindowsUpdate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Reason(
                        ABI::Windows::Management::Update::WindowsUpdateAttentionRequiredReason* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateAttentionRequiredReasonChangedEventArgs = __uuidof(IWindowsUpdateAttentionRequiredReasonChangedEventArgs);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateGetAdministratorResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateGetAdministratorResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateGetAdministratorResult[] = L"Windows.Management.Update.IWindowsUpdateGetAdministratorResult";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("bb39ffc4-2c42-5b1c-8995-343341c92c50")
                IWindowsUpdateGetAdministratorResult : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Administrator(
                        ABI::Windows::Management::Update::IWindowsUpdateAdministrator** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Status(
                        ABI::Windows::Management::Update::WindowsUpdateAdministratorStatus* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateGetAdministratorResult = __uuidof(IWindowsUpdateGetAdministratorResult);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateItem
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateItem
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateItem[] = L"Windows.Management.Update.IWindowsUpdateItem";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("b222e44a-49b6-59bf-a033-ef617cd73a98")
                IWindowsUpdateItem : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_UpdateId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Timestamp(
                        ABI::Windows::Foundation::DateTime* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MoreInfoUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Category(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Operation(
                        HSTRING* value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateItem = __uuidof(IWindowsUpdateItem);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManager
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManager[] = L"Windows.Management.Update.IWindowsUpdateManager";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("5dd966c0-a71a-5602-bbd0-09a70e4573fa")
                IWindowsUpdateManager : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE add_ScanningStateChanged(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScanningStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_WorkingStateChanged(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_WorkingStateChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ProgressChanged(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ProgressChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_AttentionRequiredReasonChanged(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_AttentionRequiredReasonChanged(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ActionCompleted(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ActionCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE add_ScanCompleted(
                        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs* handler,
                        EventRegistrationToken* token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE remove_ScanCompleted(
                        EventRegistrationToken token
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsScanning(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_IsWorking(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_LastSuccessfulScanTimestamp(
                        __FIReference_1_Windows__CFoundation__CDateTime** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetApplicableUpdates(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMostRecentCompletedUpdates(
                        INT32 count,
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetMostRecentCompletedUpdatesAsync(
                        INT32 count,
                        __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** operation
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE StartScan(
                        boolean userInitiated
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateManager = __uuidof(IWindowsUpdateManager);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManager2
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManager2[] = L"Windows.Management.Update.IWindowsUpdateManager2";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("564e7683-bd21-57a4-b17f-7bf6350f4c75")
                IWindowsUpdateManager2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE GetProvider(
                        HSTRING id,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateProvider** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderIds(
                        UINT32* valueLength,
                        HSTRING** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE GetApplicableSoftwareUpdates(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate** result
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE PerformScan(
                        ABI::Windows::Management::Update::IWindowsUpdateManagerScanOptions* options,
                        ABI::Windows::Management::Update::IWindowsSoftwareUpdateScanResult** result
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateManager2 = __uuidof(IWindowsUpdateManager2);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerFactory[] = L"Windows.Management.Update.IWindowsUpdateManagerFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("1b394df8-decb-5f44-b47c-6ccf3bcfdb37")
                IWindowsUpdateManagerFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING clientId,
                        ABI::Windows::Management::Update::IWindowsUpdateManager** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateManagerFactory = __uuidof(IWindowsUpdateManagerFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerFactory2
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerFactory2[] = L"Windows.Management.Update.IWindowsUpdateManagerFactory2";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("ba08d663-d160-59b9-9898-97a186ad52ea")
                IWindowsUpdateManagerFactory2 : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING clientId,
                        UINT32 providerIdFilterLength,
                        HSTRING* providerIdFilter,
                        ABI::Windows::Management::Update::IWindowsUpdateManager** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateManagerFactory2 = __uuidof(IWindowsUpdateManagerFactory2);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerScanOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManagerScanOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerScanOptions[] = L"Windows.Management.Update.IWindowsUpdateManagerScanOptions";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("b7c30113-5e4b-59d8-99ad-f58d67b2aefc")
                IWindowsUpdateManagerScanOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_IsUserInitiated(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_IsUserInitiated(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_AllowBypassThrottling(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_AllowBypassThrottling(
                        boolean value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_PerformUpdateActions(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_PerformUpdateActions(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateManagerScanOptions = __uuidof(IWindowsUpdateManagerScanOptions);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerScanOptionsFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManagerScanOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerScanOptionsFactory[] = L"Windows.Management.Update.IWindowsUpdateManagerScanOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("1a0f9198-f18d-5cfd-8cb9-08f3fb74da70")
                IWindowsUpdateManagerScanOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        boolean isUserInitiated,
                        ABI::Windows::Management::Update::IWindowsUpdateManagerScanOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateManagerScanOptionsFactory = __uuidof(IWindowsUpdateManagerScanOptionsFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateProgressChangedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateProgressChangedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("bbfbdeeb-94c8-5aa7-b0fb-66c67c233b0a")
                IWindowsUpdateProgressChangedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Update(
                        ABI::Windows::Management::Update::IWindowsUpdate** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ActionProgress(
                        ABI::Windows::Management::Update::IWindowsUpdateActionProgress** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateProgressChangedEventArgs = __uuidof(IWindowsUpdateProgressChangedEventArgs);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateRestartRequestOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateRestartRequestOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateRestartRequestOptions[] = L"Windows.Management.Update.IWindowsUpdateRestartRequestOptions";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("38cfb7d3-4188-5222-905c-6c4443c951ee")
                IWindowsUpdateRestartRequestOptions : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_Title(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Title(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Description(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_Description(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_MoreInfoUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass** value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_MoreInfoUrl(
                        ABI::Windows::Foundation::IUriRuntimeClass* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ComplianceDeadlineInDays(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ComplianceDeadlineInDays(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ComplianceGracePeriodInDays(
                        INT32* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_ComplianceGracePeriodInDays(
                        INT32 value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OrganizationName(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OrganizationName(
                        HSTRING value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_OptOutOfAutoReboot(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE put_OptOutOfAutoReboot(
                        boolean value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateRestartRequestOptions = __uuidof(IWindowsUpdateRestartRequestOptions);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateRestartRequestOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateRestartRequestOptionsFactory[] = L"Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("75f41d04-0e17-50d0-8c15-6b9d0539b3a9")
                IWindowsUpdateRestartRequestOptionsFactory : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                        HSTRING title,
                        HSTRING description,
                        ABI::Windows::Foundation::IUriRuntimeClass* moreInfoUrl,
                        INT32 complianceDeadlineInDays,
                        INT32 complianceGracePeriodInDays,
                        ABI::Windows::Management::Update::IWindowsUpdateRestartRequestOptions** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateRestartRequestOptionsFactory = __uuidof(IWindowsUpdateRestartRequestOptionsFactory);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateScanCompletedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateScanCompletedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs";
namespace ABI {
    namespace Windows {
        namespace Management {
            namespace Update {
                MIDL_INTERFACE("95b6953e-ba5c-5fe8-b115-12de184a6bb0")
                IWindowsUpdateScanCompletedEventArgs : public IInspectable
                {
                public:
                    virtual HRESULT STDMETHODCALLTYPE get_ProviderId(
                        HSTRING* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Succeeded(
                        boolean* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                        HRESULT* value
                        ) = 0;
                    virtual HRESULT STDMETHODCALLTYPE get_Updates(
                        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate** value
                        ) = 0;
                };

                MIDL_CONST_ID IID& IID_IWindowsUpdateScanCompletedEventArgs = __uuidof(IWindowsUpdateScanCompletedEventArgs);
            } /* Update */
        } /* Management */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.PreviewBuildsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Update.IPreviewBuildsManagerStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IPreviewBuildsManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Management_Update_PreviewBuildsManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_PreviewBuildsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_PreviewBuildsManager[] = L"Windows.Management.Update.PreviewBuildsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Management.Update.PreviewBuildsState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IPreviewBuildsState ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Management_Update_PreviewBuildsState_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_PreviewBuildsState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_PreviewBuildsState[] = L"Windows.Management.Update.PreviewBuildsState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdate ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdate_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdate[] = L"Windows.Management.Update.WindowsSoftwareUpdate";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateActionInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateActionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateActionInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateActionInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateActionProgress ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionProgress_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionProgress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateActionProgress[] = L"Windows.Management.Update.WindowsSoftwareUpdateActionProgress";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateActionResultInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionResultInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionResultInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateActionResultInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateAppPackageInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateAppPackageInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateAppPackageInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateApprovalInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateApprovalInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateApprovalInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateExecutionInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateExecutionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateExecutionInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateLocalizationInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateLocalizationInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateLocalizationInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalActionInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalActionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateOptionalActionInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateOptionalInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProvider
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateProviderFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProvider_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProvider[] = L"Windows.Management.Update.WindowsSoftwareUpdateProvider";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResultFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderActionResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderActionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProviderActionResult[] = L"Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProviderPayloadFileInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderPayloadFileInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderPayloadFileInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProviderPayloadFileInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProviderStatus
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateProviderStatusFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProviderStatus ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderStatus_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProviderStatus[] = L"Windows.Management.Update.WindowsSoftwareUpdateProviderStatus";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateResultFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateResult[] = L"Windows.Management.Update.WindowsSoftwareUpdateResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateScanResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateScanResultFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateScanResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateScanResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateScanResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateScanResult[] = L"Windows.Management.Update.WindowsSoftwareUpdateScanResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateVersion
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateVersionFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateVersion ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateVersion_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateVersion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateVersion[] = L"Windows.Management.Update.WindowsSoftwareUpdateVersion";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdate ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdate_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdate[] = L"Windows.Management.Update.WindowsUpdate";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateActionCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateActionCompletedEventArgs[] = L"Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateActionProgress ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionProgress_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionProgress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateActionProgress[] = L"Windows.Management.Update.WindowsUpdateActionProgress";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateActionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateActionResult[] = L"Windows.Management.Update.WindowsUpdateActionResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateAdministrator
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Update.IWindowsUpdateAdministratorStatics interface starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateAdministrator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAdministrator_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAdministrator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateAdministrator[] = L"Windows.Management.Update.WindowsUpdateAdministrator";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateApprovalData
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateApprovalData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateApprovalData_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateApprovalData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateApprovalData[] = L"Windows.Management.Update.WindowsUpdateApprovalData";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateAttentionRequiredInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateAttentionRequiredInfo[] = L"Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredReasonChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredReasonChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateAttentionRequiredReasonChangedEventArgs[] = L"Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateGetAdministratorResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateGetAdministratorResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateGetAdministratorResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateGetAdministratorResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateGetAdministratorResult[] = L"Windows.Management.Update.WindowsUpdateGetAdministratorResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateItem
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateItem_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateItem[] = L"Windows.Management.Update.WindowsUpdateItem";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateManager
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateManagerFactory2 interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateManagerFactory interface starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateManager ** Default Interface **
 *    Windows.Management.Update.IWindowsUpdateManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateManager[] = L"Windows.Management.Update.WindowsUpdateManager";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateManagerScanOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateManagerScanOptionsFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateManagerScanOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManagerScanOptions_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManagerScanOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateManagerScanOptions[] = L"Windows.Management.Update.WindowsUpdateManagerScanOptions";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateProgressChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateProgressChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateProgressChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateProgressChangedEventArgs[] = L"Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateRestartRequestOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory interface starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateRestartRequestOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateRestartRequestOptions_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateRestartRequestOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateRestartRequestOptions[] = L"Windows.Management.Update.WindowsUpdateRestartRequestOptions";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateScanCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateScanCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateScanCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateScanCompletedEventArgs[] = L"Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2 __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2 __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs;

#endif // ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_FWD_DEFINED__

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

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

typedef struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl;

interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

typedef struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl;

interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

typedef struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl;

interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate;

typedef struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl;

interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate;

typedef struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl;

interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType;

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType;

typedef struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType* result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType* items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl;

interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType;

typedef struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl;

interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo;

typedef struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl;

interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo;

typedef struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl;

interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo;

typedef struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl;

interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo;

typedef struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        __FIIterator_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl;

interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate;

typedef struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl;

interface __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate
{
    CONST_VTBL struct __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate;

typedef struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        __FIIterator_1_Windows__CManagement__CUpdate__CWindowsUpdate** result);

    END_INTERFACE
} __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl;

interface __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate
{
    CONST_VTBL struct __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate;

typedef struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl;

interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType;

typedef struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        UINT32 index,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType* result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType* items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl;

interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionTypeVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo;

typedef struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl;

interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo;

typedef struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl;

interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate;

typedef struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        UINT32 index,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl;

interface __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate
{
    CONST_VTBL struct __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

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

#if !defined(____FIReference_1_GUID_INTERFACE_DEFINED__)
#define ____FIReference_1_GUID_INTERFACE_DEFINED__

typedef interface __FIReference_1_GUID __FIReference_1_GUID;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_GUID;

typedef struct __FIReference_1_GUIDVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_GUID* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_GUID* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_GUID* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_GUID* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_GUID* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_GUID* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_GUID* This,
        GUID* result);

    END_INTERFACE
} __FIReference_1_GUIDVtbl;

interface __FIReference_1_GUID
{
    CONST_VTBL struct __FIReference_1_GUIDVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_GUID_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_GUID_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_GUID_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_GUID_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_GUID_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_GUID_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_GUID_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_GUID_INTERFACE_DEFINED__

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

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateRestartReason __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateRestartReason;

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_INTERFACE_DEFINED__)
#define ____FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_INTERFACE_DEFINED__

typedef interface __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason;

typedef struct __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReasonVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Value)(__FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateRestartReason* result);

    END_INTERFACE
} __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReasonVtbl;

interface __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason
{
    CONST_VTBL struct __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReasonVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_get_Value(This, result) \
    ((This)->lpVtbl->get_Value(This, result))

#endif /* COBJMACROS */

#endif // ____FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectableVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* sender,
        IInspectable* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectableVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectableVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* sender,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* sender,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* sender,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* sender,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet;

#endif // ____x_ABI_CWindows_CFoundation_CCollections_CIPropertySet_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIUriRuntimeClass __x_ABI_CWindows_CFoundation_CIUriRuntimeClass;

#endif // ____x_ABI_CWindows_CFoundation_CIUriRuntimeClass_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionResult __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionResult;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateArchitecture __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateArchitecture;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateInstallationType __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateInstallationType;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderRegistrationType __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderRegistrationType;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderTrustState __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderTrustState;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderType __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderType;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorOptions __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorOptions;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorStatus __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorStatus;

typedef enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAttentionRequiredReason __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAttentionRequiredReason;

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionResult
{
    WindowsSoftwareUpdateActionResult_Succeeded = 0,
    WindowsSoftwareUpdateActionResult_Continue = 1,
    WindowsSoftwareUpdateActionResult_Failed = 2,
    WindowsSoftwareUpdateActionResult_Canceled = 3,
    WindowsSoftwareUpdateActionResult_Removed = 4,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateActionType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType
{
    WindowsSoftwareUpdateActionType_Download = 0,
    WindowsSoftwareUpdateActionType_Install = 1,
    WindowsSoftwareUpdateActionType_Deploy = 2,
    WindowsSoftwareUpdateActionType_Reboot = 3,
    WindowsSoftwareUpdateActionType_AppRestart = 4,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateArchitecture
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateArchitecture
{
    WindowsSoftwareUpdateArchitecture_Neutral = 0,
    WindowsSoftwareUpdateArchitecture_X86 = 1,
    WindowsSoftwareUpdateArchitecture_X64 = 2,
    WindowsSoftwareUpdateArchitecture_Arm = 3,
    WindowsSoftwareUpdateArchitecture_Arm64 = 4,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateInstallationType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateInstallationType
{
    WindowsSoftwareUpdateInstallationType_WindowsUpdate = 0,
    WindowsSoftwareUpdateInstallationType_AppPackage = 1,
    WindowsSoftwareUpdateInstallationType_Executable = 2,
    WindowsSoftwareUpdateInstallationType_Powershell = 3,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateProviderRegistrationType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderRegistrationType
{
    WindowsSoftwareUpdateProviderRegistrationType_None = 0,
    WindowsSoftwareUpdateProviderRegistrationType_System = 1,
    WindowsSoftwareUpdateProviderRegistrationType_Windows = 2,
    WindowsSoftwareUpdateProviderRegistrationType_Pending = 3,
    WindowsSoftwareUpdateProviderRegistrationType_Registered = 4,
    WindowsSoftwareUpdateProviderRegistrationType_Unregistered = 5,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateProviderTrustState
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderTrustState
{
    WindowsSoftwareUpdateProviderTrustState_SignedTrusted = 0,
    WindowsSoftwareUpdateProviderTrustState_SignedUntrusted = 1,
    WindowsSoftwareUpdateProviderTrustState_Unsigned = 2,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateProviderType
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderType
{
    WindowsSoftwareUpdateProviderType_WindowsUpdate = 0,
    WindowsSoftwareUpdateProviderType_Executable = 1,
    WindowsSoftwareUpdateProviderType_Powershell = 2,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsSoftwareUpdateRestartReason
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateRestartReason
{
    WindowsSoftwareUpdateRestartReason_None = 0,
    WindowsSoftwareUpdateRestartReason_System = 1,
    WindowsSoftwareUpdateRestartReason_AppClose = 2,
    WindowsSoftwareUpdateRestartReason_AppRestart = 3,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.Management.Update.WindowsUpdateAdministratorOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorOptions
{
    WindowsUpdateAdministratorOptions_None = 0,
    WindowsUpdateAdministratorOptions_RequireAdministratorApprovalForScans = 0x1,
    WindowsUpdateAdministratorOptions_RequireAdministratorApprovalForUpdates = 0x2,
    WindowsUpdateAdministratorOptions_RequireAdministratorApprovalForActions = 0x4,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Management.Update.WindowsUpdateAdministratorStatus
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorStatus
{
    WindowsUpdateAdministratorStatus_Succeeded = 0,
    WindowsUpdateAdministratorStatus_NoAdministratorRegistered = 1,
    WindowsUpdateAdministratorStatus_OtherAdministratorIsRegistered = 2,
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.Management.Update.WindowsUpdateAttentionRequiredReason
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAttentionRequiredReason
{
    WindowsUpdateAttentionRequiredReason_None = 0,
    WindowsUpdateAttentionRequiredReason_SeekerUpdate = 1,
    WindowsUpdateAttentionRequiredReason_ReadyToReboot = 2,
    WindowsUpdateAttentionRequiredReason_NeedNonMeteredNetwork = 3,
    WindowsUpdateAttentionRequiredReason_NeedUserAgreementForMeteredNetwork = 4,
    WindowsUpdateAttentionRequiredReason_NeedNetwork = 5,
    WindowsUpdateAttentionRequiredReason_NeedMoreSpace = 6,
    WindowsUpdateAttentionRequiredReason_BatterySaverEnabled = 7,
    WindowsUpdateAttentionRequiredReason_NeedUserInteraction = 8,
    WindowsUpdateAttentionRequiredReason_NeedUserAgreementForPolicy = 9,
    WindowsUpdateAttentionRequiredReason_CompatibilityError = 10,
    WindowsUpdateAttentionRequiredReason_NeedUserInteractionForEula = 11,
    WindowsUpdateAttentionRequiredReason_NeedUserInteractionForCta = 12,
    WindowsUpdateAttentionRequiredReason_Regulated = 13,
    WindowsUpdateAttentionRequiredReason_ExternalReboot = 14,
    WindowsUpdateAttentionRequiredReason_OtherUpdate = 15,
    WindowsUpdateAttentionRequiredReason_BlockedByProvider = 16,
    WindowsUpdateAttentionRequiredReason_BlockedByPostRebootFailure = 17,
    WindowsUpdateAttentionRequiredReason_UserEngaged = 18,
    WindowsUpdateAttentionRequiredReason_BlockedByBattery = 19,
    WindowsUpdateAttentionRequiredReason_Exclusivity = 20,
    WindowsUpdateAttentionRequiredReason_BlockedBySerialization = 21,
    WindowsUpdateAttentionRequiredReason_ConflictClass = 22,
    WindowsUpdateAttentionRequiredReason_BlockedByAdminApproval = 23,
    WindowsUpdateAttentionRequiredReason_BlockedByTooManyAttempts = 24,
    WindowsUpdateAttentionRequiredReason_BlockedByFailure = 25,
    WindowsUpdateAttentionRequiredReason_Demotion = 26,
    WindowsUpdateAttentionRequiredReason_BlockedByActiveHours = 27,
    WindowsUpdateAttentionRequiredReason_ScheduledForMaintenance = 28,
    WindowsUpdateAttentionRequiredReason_PolicyScheduledInstallTime = 29,
    WindowsUpdateAttentionRequiredReason_BlockedByOobe = 30,
    WindowsUpdateAttentionRequiredReason_DeferredDuringOobe = 31,
    WindowsUpdateAttentionRequiredReason_DeferredForSustainableTime = 32,
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
    WindowsUpdateAttentionRequiredReason_BlockedByAppClose = 33,
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
    WindowsUpdateAttentionRequiredReason_BlockedByAppRestart = 34,
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20001
    WindowsUpdateAttentionRequiredReason_OtherUpdateReverting = 35,
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20001
};
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IPreviewBuildsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.PreviewBuildsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IPreviewBuildsManager[] = L"Windows.Management.Update.IPreviewBuildsManager";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ArePreviewBuildsAllowed)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_ArePreviewBuildsAllowed)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* GetCurrentState)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState** result);
    HRESULT (STDMETHODCALLTYPE* SyncAsync)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager* This,
        __FIAsyncOperation_1_boolean** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_get_ArePreviewBuildsAllowed(This, value) \
    ((This)->lpVtbl->get_ArePreviewBuildsAllowed(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_put_ArePreviewBuildsAllowed(This, value) \
    ((This)->lpVtbl->put_ArePreviewBuildsAllowed(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_GetCurrentState(This, result) \
    ((This)->lpVtbl->GetCurrentState(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_SyncAsync(This, result) \
    ((This)->lpVtbl->SyncAsync(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Management.Update.IPreviewBuildsManagerStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.PreviewBuildsManager
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IPreviewBuildsManagerStatics[] = L"Windows.Management.Update.IPreviewBuildsManagerStatics";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDefault)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManager** value);
    HRESULT (STDMETHODCALLTYPE* IsSupported)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics* This,
        boolean* result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_GetDefault(This, value) \
    ((This)->lpVtbl->GetDefault(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_IsSupported(This, result) \
    ((This)->lpVtbl->IsSupported(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsManagerStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Management.Update.IPreviewBuildsState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.PreviewBuildsState
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IPreviewBuildsState[] = L"Windows.Management.Update.IPreviewBuildsState";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsStateVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIPreviewBuildsState_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdate
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdate[] = L"Windows.Management.Update.IWindowsSoftwareUpdate";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InstallationType)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateInstallationType* value);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MoreInfoUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_DownloadSizeInBytes)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_InstallSizeInBytes)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_SourceVersion)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion** value);
    HRESULT (STDMETHODCALLTYPE* get_TargetVersion)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion** value);
    HRESULT (STDMETHODCALLTYPE* get_ProductCode)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __FIReference_1_GUID** value);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* Approve)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* approvalInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* ApproveCurrentAction)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        boolean approve,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* get_CurrentAction)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ActionResultInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ApprovalInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ApprovedActions)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateActionType** value);
    HRESULT (STDMETHODCALLTYPE* get_AttentionRequiredInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ActionProgress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress** value);
    HRESULT (STDMETHODCALLTYPE* get_RestartReason)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __FIReference_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateRestartReason** value);
    HRESULT (STDMETHODCALLTYPE* get_AppPackageInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ExecutionInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_OptionalInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_InstallationType(This, value) \
    ((This)->lpVtbl->get_InstallationType(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_UpdateId(This, value) \
    ((This)->lpVtbl->get_UpdateId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_MoreInfoUrl(This, value) \
    ((This)->lpVtbl->get_MoreInfoUrl(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_DownloadSizeInBytes(This, value) \
    ((This)->lpVtbl->get_DownloadSizeInBytes(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_InstallSizeInBytes(This, value) \
    ((This)->lpVtbl->get_InstallSizeInBytes(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_SourceVersion(This, value) \
    ((This)->lpVtbl->get_SourceVersion(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_TargetVersion(This, value) \
    ((This)->lpVtbl->get_TargetVersion(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ProductCode(This, value) \
    ((This)->lpVtbl->get_ProductCode(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_Approve(This, approvalInfo, result) \
    ((This)->lpVtbl->Approve(This, approvalInfo, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_ApproveCurrentAction(This, approve, result) \
    ((This)->lpVtbl->ApproveCurrentAction(This, approve, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_CurrentAction(This, value) \
    ((This)->lpVtbl->get_CurrentAction(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ActionResultInfo(This, value) \
    ((This)->lpVtbl->get_ActionResultInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ApprovalInfo(This, value) \
    ((This)->lpVtbl->get_ApprovalInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ApprovedActions(This, value) \
    ((This)->lpVtbl->get_ApprovedActions(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_AttentionRequiredInfo(This, value) \
    ((This)->lpVtbl->get_AttentionRequiredInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ActionProgress(This, value) \
    ((This)->lpVtbl->get_ActionProgress(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_RestartReason(This, value) \
    ((This)->lpVtbl->get_RestartReason(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_AppPackageInfo(This, value) \
    ((This)->lpVtbl->get_AppPackageInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_ExecutionInfo(This, value) \
    ((This)->lpVtbl->get_ExecutionInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_get_OptionalInfo(This, value) \
    ((This)->lpVtbl->get_OptionalInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FileName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FileArguments)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ActionType)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_get_FileName(This, value) \
    ((This)->lpVtbl->get_FileName(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_get_FileArguments(This, value) \
    ((This)->lpVtbl->get_FileArguments(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_get_ActionType(This, value) \
    ((This)->lpVtbl->get_ActionType(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory* This,
        HSTRING fileName,
        HSTRING fileArguments,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionType actionType,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_CreateInstance(This, fileName, fileArguments, actionType, value) \
    ((This)->lpVtbl->CreateInstance(This, fileName, fileArguments, actionType, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionProgress
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionProgress[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionProgress";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentProgress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_TotalProgress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgressVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_get_CurrentProgress(This, value) \
    ((This)->lpVtbl->get_CurrentProgress(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_get_TotalProgress(This, value) \
    ((This)->lpVtbl->get_TotalProgress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionProgress_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateActionResultInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateActionResultInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateActionResultInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ResultCode)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_get_ResultCode(This, value) \
    ((This)->lpVtbl->get_ResultCode(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionResultInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateAppPackageInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PackageArchitecture)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateArchitecture* value);
    HRESULT (STDMETHODCALLTYPE* get_InstallUri)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_get_PackageArchitecture(This, value) \
    ((This)->lpVtbl->get_PackageArchitecture(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_get_InstallUri(This, value) \
    ((This)->lpVtbl->get_InstallUri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateAppPackageInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory* This,
        HSTRING packageFamilyName,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateArchitecture packageArchitecture,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* installUri,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_CreateInstance(This, packageFamilyName, packageArchitecture, installUri, value) \
    ((This)->lpVtbl->CreateInstance(This, packageFamilyName, packageArchitecture, installUri, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateApprovalInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_UserInitiated)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AppClosure)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MeteredNetwork)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Seeker)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_get_UserInitiated(This, value) \
    ((This)->lpVtbl->get_UserInitiated(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_get_AppClosure(This, value) \
    ((This)->lpVtbl->get_AppClosure(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_get_MeteredNetwork(This, value) \
    ((This)->lpVtbl->get_MeteredNetwork(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_get_Seeker(This, value) \
    ((This)->lpVtbl->get_Seeker(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateApprovalInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory* This,
        boolean userInitiated,
        boolean appClosure,
        boolean meteredNetwork,
        boolean seeker,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_CreateInstance(This, userInitiated, appClosure, meteredNetwork, seeker, value) \
    ((This)->lpVtbl->CreateInstance(This, userInitiated, appClosure, meteredNetwork, seeker, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateApprovalInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateExecutionInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DownloadInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_InstallInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_DeployInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_OptionalActionInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_get_DownloadInfo(This, value) \
    ((This)->lpVtbl->get_DownloadInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_get_InstallInfo(This, value) \
    ((This)->lpVtbl->get_InstallInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_get_DeployInfo(This, value) \
    ((This)->lpVtbl->get_DeployInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_get_OptionalActionInfo(This, value) \
    ((This)->lpVtbl->get_OptionalActionInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateExecutionInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* downloadInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* installInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* actions,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance2)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* deployInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* actions,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_CreateInstance(This, downloadInfo, installInfo, actions, value) \
    ((This)->lpVtbl->CreateInstance(This, downloadInfo, installInfo, actions, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_CreateInstance2(This, deployInfo, actions, value) \
    ((This)->lpVtbl->CreateInstance2(This, deployInfo, actions, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdate
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This,
        HSTRING providerId,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateInstallationType installationType,
        HSTRING updateId,
        HSTRING title,
        HSTRING description,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* moreInfoUrl,
        UINT64 downloadSizeInBytes,
        UINT64 installSizeInBytes,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* sourceVersion,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* targetVersion,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* appPackageInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* executionInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* optionalInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance2)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory* This,
        HSTRING providerId,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateInstallationType installationType,
        HSTRING updateId,
        HSTRING title,
        HSTRING description,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* moreInfoUrl,
        UINT64 downloadSizeInBytes,
        UINT64 installSizeInBytes,
        __FIReference_1_GUID* productCode,
        HSTRING packageFamilyName,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* sourceVersion,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* targetVersion,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateAppPackageInfo* appPackageInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateExecutionInfo* executionInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* optionalInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdate** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_CreateInstance(This, providerId, installationType, updateId, title, description, moreInfoUrl, downloadSizeInBytes, installSizeInBytes, sourceVersion, targetVersion, appPackageInfo, executionInfo, optionalInfo, value) \
    ((This)->lpVtbl->CreateInstance(This, providerId, installationType, updateId, title, description, moreInfoUrl, downloadSizeInBytes, installSizeInBytes, sourceVersion, targetVersion, appPackageInfo, executionInfo, optionalInfo, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_CreateInstance2(This, providerId, installationType, updateId, title, description, moreInfoUrl, downloadSizeInBytes, installSizeInBytes, productCode, packageFamilyName, sourceVersion, targetVersion, appPackageInfo, executionInfo, optionalInfo, value) \
    ((This)->lpVtbl->CreateInstance2(This, providerId, installationType, updateId, title, description, moreInfoUrl, downloadSizeInBytes, installSizeInBytes, productCode, packageFamilyName, sourceVersion, targetVersion, appPackageInfo, executionInfo, optionalInfo, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateLocalizationInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LanguageId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MoreInfoUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_get_LanguageId(This, value) \
    ((This)->lpVtbl->get_LanguageId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_get_MoreInfoUrl(This, value) \
    ((This)->lpVtbl->get_MoreInfoUrl(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateLocalizationInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory* This,
        UINT32 languageId,
        HSTRING title,
        HSTRING description,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* moreInfoUrl,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_CreateInstance(This, languageId, title, description, moreInfoUrl, value) \
    ((This)->lpVtbl->CreateInstance(This, languageId, title, description, moreInfoUrl, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateLocalizationInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalActionInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_CloseAndDeployInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_CloseAndInstallInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_CloseAndRestartInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_get_CloseAndDeployInfo(This, value) \
    ((This)->lpVtbl->get_CloseAndDeployInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_get_CloseAndInstallInfo(This, value) \
    ((This)->lpVtbl->get_CloseAndInstallInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_get_CloseAndRestartInfo(This, value) \
    ((This)->lpVtbl->get_CloseAndRestartInfo(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalActionInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* closeAndDeployInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* closeAndInstallInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateActionInfo* closeAndRestartInfo,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_CreateInstance(This, closeAndDeployInfo, closeAndInstallInfo, closeAndRestartInfo, value) \
    ((This)->lpVtbl->CreateInstance(This, closeAndDeployInfo, closeAndInstallInfo, closeAndRestartInfo, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalActionInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LocalizationInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ComplianceDeadlineInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* get_ComplianceGracePeriodInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo* This,
        __FIReference_1_int** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_get_LocalizationInfo(This, value) \
    ((This)->lpVtbl->get_LocalizationInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_get_ComplianceDeadlineInDays(This, value) \
    ((This)->lpVtbl->get_ComplianceDeadlineInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_get_ComplianceGracePeriodInDays(This, value) \
    ((This)->lpVtbl->get_ComplianceGracePeriodInDays(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfoFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateOptionalInfoFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfoFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This,
        __FIReference_1_int* complianceDeadlineInDays,
        __FIReference_1_int* complianceGracePeriodInDays,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance2)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory* This,
        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateLocalizationInfo* localizationInfo,
        __FIReference_1_int* complianceDeadlineInDays,
        __FIReference_1_int* complianceGracePeriodInDays,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfo** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_CreateInstance(This, complianceDeadlineInDays, complianceGracePeriodInDays, value) \
    ((This)->lpVtbl->CreateInstance(This, complianceDeadlineInDays, complianceGracePeriodInDays, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_CreateInstance2(This, localizationInfo, complianceDeadlineInDays, complianceGracePeriodInDays, value) \
    ((This)->lpVtbl->CreateInstance2(This, localizationInfo, complianceDeadlineInDays, complianceGracePeriodInDays, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateOptionalInfoFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProvider
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProvider
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProvider[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProvider";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Register)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* Unregister)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* Validate)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FolderPath)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CatalogFile)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ScanFileName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ScanFileArguments)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Type)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderType* value);
    HRESULT (STDMETHODCALLTYPE* get_PayloadFiles)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderPayloadFileInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_TrustState)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderTrustState* value);
    HRESULT (STDMETHODCALLTYPE* get_RegistrationType)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderRegistrationType* value);
    HRESULT (STDMETHODCALLTYPE* get_Properties)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        __x_ABI_CWindows_CFoundation_CCollections_CIPropertySet** value);
    HRESULT (STDMETHODCALLTYPE* GetPropertyValue)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider* This,
        HSTRING name,
        IInspectable** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_Register(This, result) \
    ((This)->lpVtbl->Register(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_Unregister(This, result) \
    ((This)->lpVtbl->Unregister(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_Validate(This, result) \
    ((This)->lpVtbl->Validate(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_FolderPath(This, value) \
    ((This)->lpVtbl->get_FolderPath(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_CatalogFile(This, value) \
    ((This)->lpVtbl->get_CatalogFile(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_ScanFileName(This, value) \
    ((This)->lpVtbl->get_ScanFileName(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_ScanFileArguments(This, value) \
    ((This)->lpVtbl->get_ScanFileArguments(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_Type(This, value) \
    ((This)->lpVtbl->get_Type(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_PayloadFiles(This, value) \
    ((This)->lpVtbl->get_PayloadFiles(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_TrustState(This, value) \
    ((This)->lpVtbl->get_TrustState(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_RegistrationType(This, value) \
    ((This)->lpVtbl->get_RegistrationType(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_get_Properties(This, value) \
    ((This)->lpVtbl->get_Properties(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_GetPropertyValue(This, name, result) \
    ((This)->lpVtbl->GetPropertyValue(This, name, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderActionResult[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResult";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Result)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionResult* value);
    HRESULT (STDMETHODCALLTYPE* get_RestartReason)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateRestartReason* value);
    HRESULT (STDMETHODCALLTYPE* get_ResultCode)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_get_Result(This, value) \
    ((This)->lpVtbl->get_Result(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_get_RestartReason(This, value) \
    ((This)->lpVtbl->get_RestartReason(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_get_ResultCode(This, value) \
    ((This)->lpVtbl->get_ResultCode(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResultFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderActionResultFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResultFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateActionResult actionResult,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateRestartReason restartReason,
        UINT32 resultCode,
        UINT64 extendedError,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_CreateInstance(This, actionResult, restartReason, resultCode, extendedError, value) \
    ((This)->lpVtbl->CreateInstance(This, actionResult, restartReason, resultCode, extendedError, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProvider
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory* This,
        HSTRING folderPath,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_CreateInstance(This, folderPath, value) \
    ((This)->lpVtbl->CreateInstance(This, folderPath, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderPayloadFileInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderPayloadFileInfo[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderPayloadFileInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Filename)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FileHash)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_CatalogFile)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_TrustState)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsSoftwareUpdateProviderTrustState* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_get_Filename(This, value) \
    ((This)->lpVtbl->get_Filename(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_get_FileHash(This, value) \
    ((This)->lpVtbl->get_FileHash(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_get_CatalogFile(This, value) \
    ((This)->lpVtbl->get_CatalogFile(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_get_TrustState(This, value) \
    ((This)->lpVtbl->get_TrustState(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderPayloadFileInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderStatus
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderStatus
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderStatus[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderStatus";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_CancelRequested)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsSoftwareUpdateProviderStatus_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_CancelRequested)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* SetScanResult)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        boolean succeeded,
        UINT32 resultCode,
        UINT64 extendedError,
        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* updates,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* SetActionProgress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        UINT64 current,
        UINT64 total,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);
    HRESULT (STDMETHODCALLTYPE* SetActionResult)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderActionResult* actionResult,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_add_CancelRequested(This, handler, token) \
    ((This)->lpVtbl->add_CancelRequested(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_remove_CancelRequested(This, token) \
    ((This)->lpVtbl->remove_CancelRequested(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_SetScanResult(This, succeeded, resultCode, extendedError, updates, result) \
    ((This)->lpVtbl->SetScanResult(This, succeeded, resultCode, extendedError, updates, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_SetActionProgress(This, current, total, result) \
    ((This)->lpVtbl->SetActionProgress(This, current, total, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_SetActionResult(This, actionResult, result) \
    ((This)->lpVtbl->SetActionResult(This, actionResult, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateProviderStatusFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateProviderStatus
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateProviderStatusFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateProviderStatusFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory* This,
        HSTRING providerId,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatus** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_CreateInstance(This, providerId, value) \
    ((This)->lpVtbl->CreateInstance(This, providerId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProviderStatusFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateResult[] = L"Windows.Management.Update.IWindowsSoftwareUpdateResult";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_CancelRequested)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ResultCode)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult* This,
        UINT64* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_get_CancelRequested(This, value) \
    ((This)->lpVtbl->get_CancelRequested(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_get_ResultCode(This, value) \
    ((This)->lpVtbl->get_ResultCode(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateResultFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateResultFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateResultFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        boolean succeeded,
        UINT32 resultCode,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance2)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        boolean succeeded,
        UINT32 resultCode,
        UINT64 extendedError,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance3)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory* This,
        boolean succeeded,
        boolean cancelRequested,
        UINT32 resultCode,
        UINT64 extendedError,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_CreateInstance(This, succeeded, resultCode, value) \
    ((This)->lpVtbl->CreateInstance(This, succeeded, resultCode, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_CreateInstance2(This, succeeded, resultCode, extendedError, value) \
    ((This)->lpVtbl->CreateInstance2(This, succeeded, resultCode, extendedError, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_CreateInstance3(This, succeeded, cancelRequested, resultCode, extendedError, value) \
    ((This)->lpVtbl->CreateInstance3(This, succeeded, cancelRequested, resultCode, extendedError, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateScanResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateScanResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateScanResult[] = L"Windows.Management.Update.IWindowsSoftwareUpdateScanResult";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ResultCode)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        UINT64* value);
    HRESULT (STDMETHODCALLTYPE* get_Updates)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_get_ResultCode(This, value) \
    ((This)->lpVtbl->get_ResultCode(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_get_Updates(This, value) \
    ((This)->lpVtbl->get_Updates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateScanResultFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateScanResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateScanResultFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateScanResultFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This,
        boolean succeeded,
        UINT32 resultCode,
        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* updates,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult** value);
    HRESULT (STDMETHODCALLTYPE* CreateInstance2)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory* This,
        boolean succeeded,
        UINT32 resultCode,
        UINT64 extendedError,
        __FIIterable_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate* updates,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_CreateInstance(This, succeeded, resultCode, updates, value) \
    ((This)->lpVtbl->CreateInstance(This, succeeded, resultCode, updates, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_CreateInstance2(This, succeeded, resultCode, extendedError, updates, value) \
    ((This)->lpVtbl->CreateInstance2(This, succeeded, resultCode, extendedError, updates, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResultFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateVersion
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateVersion
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateVersion[] = L"Windows.Management.Update.IWindowsSoftwareUpdateVersion";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Major)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_Minor)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_RevisionMajor)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_RevisionMinor)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion* This,
        UINT32* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_get_Major(This, value) \
    ((This)->lpVtbl->get_Major(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_get_Minor(This, value) \
    ((This)->lpVtbl->get_Minor(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_get_RevisionMajor(This, value) \
    ((This)->lpVtbl->get_RevisionMajor(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_get_RevisionMinor(This, value) \
    ((This)->lpVtbl->get_RevisionMinor(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsSoftwareUpdateVersionFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsSoftwareUpdateVersion
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsSoftwareUpdateVersionFactory[] = L"Windows.Management.Update.IWindowsSoftwareUpdateVersionFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory* This,
        UINT32 major,
        UINT32 minor,
        UINT32 revisionMajor,
        UINT32 revisionMinor,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersion** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_CreateInstance(This, major, minor, revisionMajor, revisionMinor, value) \
    ((This)->lpVtbl->CreateInstance(This, major, minor, revisionMajor, revisionMinor, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateVersionFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdate
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdate[] = L"Windows.Management.Update.IWindowsUpdate";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsFeatureUpdate)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMinorImpact)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSecurity)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCritical)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsForOS)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDriver)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsMandatory)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsUrgent)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsSeeker)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_MoreInfoUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_SupportUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_IsEulaAccepted)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_EulaText)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_AttentionRequiredInfo)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_ActionResult)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult** value);
    HRESULT (STDMETHODCALLTYPE* get_CurrentAction)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_ActionProgress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress** value);
    HRESULT (STDMETHODCALLTYPE* GetPropertyValue)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This,
        HSTRING propertyName,
        IInspectable** result);
    HRESULT (STDMETHODCALLTYPE* AcceptEula)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate* This);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_UpdateId(This, value) \
    ((This)->lpVtbl->get_UpdateId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsFeatureUpdate(This, value) \
    ((This)->lpVtbl->get_IsFeatureUpdate(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsMinorImpact(This, value) \
    ((This)->lpVtbl->get_IsMinorImpact(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsSecurity(This, value) \
    ((This)->lpVtbl->get_IsSecurity(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsCritical(This, value) \
    ((This)->lpVtbl->get_IsCritical(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsForOS(This, value) \
    ((This)->lpVtbl->get_IsForOS(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsDriver(This, value) \
    ((This)->lpVtbl->get_IsDriver(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsMandatory(This, value) \
    ((This)->lpVtbl->get_IsMandatory(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsUrgent(This, value) \
    ((This)->lpVtbl->get_IsUrgent(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsSeeker(This, value) \
    ((This)->lpVtbl->get_IsSeeker(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_MoreInfoUrl(This, value) \
    ((This)->lpVtbl->get_MoreInfoUrl(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_SupportUrl(This, value) \
    ((This)->lpVtbl->get_SupportUrl(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_IsEulaAccepted(This, value) \
    ((This)->lpVtbl->get_IsEulaAccepted(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_EulaText(This, value) \
    ((This)->lpVtbl->get_EulaText(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_AttentionRequiredInfo(This, value) \
    ((This)->lpVtbl->get_AttentionRequiredInfo(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_ActionResult(This, value) \
    ((This)->lpVtbl->get_ActionResult(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_CurrentAction(This, value) \
    ((This)->lpVtbl->get_CurrentAction(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_get_ActionProgress(This, value) \
    ((This)->lpVtbl->get_ActionProgress(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_GetPropertyValue(This, propertyName, result) \
    ((This)->lpVtbl->GetPropertyValue(This, propertyName, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_AcceptEula(This) \
    ((This)->lpVtbl->AcceptEula(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateActionCompletedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateActionCompletedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Update)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** value);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_get_Update(This, value) \
    ((This)->lpVtbl->get_Update(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateActionProgress
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateActionProgress[] = L"Windows.Management.Update.IWindowsUpdateActionProgress";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress* This,
        DOUBLE* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgressVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateActionResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateActionResult[] = L"Windows.Management.Update.IWindowsUpdateActionResult";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_Action)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResultVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_get_Action(This, value) \
    ((This)->lpVtbl->get_Action(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAdministrator
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAdministrator
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAdministrator[] = L"Windows.Management.Update.IWindowsUpdateAdministrator";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* StartAdministratorScan)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This);
    HRESULT (STDMETHODCALLTYPE* ApproveWindowsUpdateAction)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        HSTRING updateId,
        HSTRING action);
    HRESULT (STDMETHODCALLTYPE* RevokeWindowsUpdateActionApproval)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        HSTRING updateId,
        HSTRING action);
    HRESULT (STDMETHODCALLTYPE* ApproveWindowsUpdate)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        HSTRING updateId,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* approvalData);
    HRESULT (STDMETHODCALLTYPE* RevokeWindowsUpdateApproval)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        HSTRING updateId);
    HRESULT (STDMETHODCALLTYPE* GetUpdates)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_StartAdministratorScan(This) \
    ((This)->lpVtbl->StartAdministratorScan(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_ApproveWindowsUpdateAction(This, updateId, action) \
    ((This)->lpVtbl->ApproveWindowsUpdateAction(This, updateId, action))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_RevokeWindowsUpdateActionApproval(This, updateId, action) \
    ((This)->lpVtbl->RevokeWindowsUpdateActionApproval(This, updateId, action))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_ApproveWindowsUpdate(This, updateId, approvalData) \
    ((This)->lpVtbl->ApproveWindowsUpdate(This, updateId, approvalData))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_RevokeWindowsUpdateApproval(This, updateId) \
    ((This)->lpVtbl->RevokeWindowsUpdateApproval(This, updateId))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_GetUpdates(This, result) \
    ((This)->lpVtbl->GetUpdates(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAdministratorStatics
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAdministrator
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAdministratorStatics[] = L"Windows.Management.Update.IWindowsUpdateAdministratorStatics";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetRegisteredAdministrator)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        HSTRING organizationName,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult** result);
    HRESULT (STDMETHODCALLTYPE* RegisterForAdministration)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        HSTRING organizationName,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorOptions options,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorStatus* result);
    HRESULT (STDMETHODCALLTYPE* UnregisterForAdministration)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        HSTRING organizationName,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorStatus* result);
    HRESULT (STDMETHODCALLTYPE* GetRegisteredAdministratorName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* RequestRestart)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* restartOptions,
        HSTRING* result);
    HRESULT (STDMETHODCALLTYPE* CancelRestartRequest)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics* This,
        HSTRING requestRestartToken);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStaticsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_GetRegisteredAdministrator(This, organizationName, result) \
    ((This)->lpVtbl->GetRegisteredAdministrator(This, organizationName, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_RegisterForAdministration(This, organizationName, options, result) \
    ((This)->lpVtbl->RegisterForAdministration(This, organizationName, options, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_UnregisterForAdministration(This, organizationName, result) \
    ((This)->lpVtbl->UnregisterForAdministration(This, organizationName, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_GetRegisteredAdministratorName(This, result) \
    ((This)->lpVtbl->GetRegisteredAdministratorName(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_RequestRestart(This, restartOptions, result) \
    ((This)->lpVtbl->RequestRestart(This, restartOptions, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_CancelRestartRequest(This, requestRestartToken) \
    ((This)->lpVtbl->CancelRestartRequest(This, requestRestartToken))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministratorStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateApprovalData
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateApprovalData
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateApprovalData[] = L"Windows.Management.Update.IWindowsUpdateApprovalData";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalDataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Seeker)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_Seeker)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AllowDownloadOnMetered)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_AllowDownloadOnMetered)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ComplianceDeadlineInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_ComplianceDeadlineInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_ComplianceGracePeriodInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_int** value);
    HRESULT (STDMETHODCALLTYPE* put_ComplianceGracePeriodInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_int* value);
    HRESULT (STDMETHODCALLTYPE* get_OptOutOfAutoReboot)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_boolean** value);
    HRESULT (STDMETHODCALLTYPE* put_OptOutOfAutoReboot)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData* This,
        __FIReference_1_boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalDataVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalDataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_get_Seeker(This, value) \
    ((This)->lpVtbl->get_Seeker(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_put_Seeker(This, value) \
    ((This)->lpVtbl->put_Seeker(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_get_AllowDownloadOnMetered(This, value) \
    ((This)->lpVtbl->get_AllowDownloadOnMetered(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_put_AllowDownloadOnMetered(This, value) \
    ((This)->lpVtbl->put_AllowDownloadOnMetered(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_get_ComplianceDeadlineInDays(This, value) \
    ((This)->lpVtbl->get_ComplianceDeadlineInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_put_ComplianceDeadlineInDays(This, value) \
    ((This)->lpVtbl->put_ComplianceDeadlineInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_get_ComplianceGracePeriodInDays(This, value) \
    ((This)->lpVtbl->get_ComplianceGracePeriodInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_put_ComplianceGracePeriodInDays(This, value) \
    ((This)->lpVtbl->put_ComplianceGracePeriodInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_get_OptOutOfAutoReboot(This, value) \
    ((This)->lpVtbl->get_OptOutOfAutoReboot(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_put_OptOutOfAutoReboot(This, value) \
    ((This)->lpVtbl->put_OptOutOfAutoReboot(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateApprovalData_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAttentionRequiredInfo
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAttentionRequiredInfo[] = L"Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAttentionRequiredReason* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfoVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateAttentionRequiredReasonChangedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Update)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** value);
    HRESULT (STDMETHODCALLTYPE* get_Reason)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAttentionRequiredReason* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_get_Update(This, value) \
    ((This)->lpVtbl->get_Update(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_get_Reason(This, value) \
    ((This)->lpVtbl->get_Reason(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAttentionRequiredReasonChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateGetAdministratorResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateGetAdministratorResult
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateGetAdministratorResult[] = L"Windows.Management.Update.IWindowsUpdateGetAdministratorResult";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Administrator)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateAdministrator** value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult* This,
        enum __x_ABI_CWindows_CManagement_CUpdate_CWindowsUpdateAdministratorStatus* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResultVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_get_Administrator(This, value) \
    ((This)->lpVtbl->get_Administrator(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateGetAdministratorResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateItem
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateItem
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateItem[] = L"Windows.Management.Update.IWindowsUpdateItem";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItemVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Timestamp)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MoreInfoUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_Category)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Operation)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItemVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItemVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_UpdateId(This, value) \
    ((This)->lpVtbl->get_UpdateId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_Timestamp(This, value) \
    ((This)->lpVtbl->get_Timestamp(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_MoreInfoUrl(This, value) \
    ((This)->lpVtbl->get_MoreInfoUrl(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_Category(This, value) \
    ((This)->lpVtbl->get_Category(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_get_Operation(This, value) \
    ((This)->lpVtbl->get_Operation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateItem_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManager
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManager[] = L"Windows.Management.Update.IWindowsUpdateManager";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_ScanningStateChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ScanningStateChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_WorkingStateChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_IInspectable* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_WorkingStateChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ProgressChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateProgressChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ProgressChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_AttentionRequiredReasonChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateAttentionRequiredReasonChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_AttentionRequiredReasonChanged)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ActionCompleted)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateActionCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ActionCompleted)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_ScanCompleted)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FITypedEventHandler_2_Windows__CManagement__CUpdate__CWindowsUpdateManager_Windows__CManagement__CUpdate__CWindowsUpdateScanCompletedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_ScanCompleted)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* get_IsScanning)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsWorking)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_LastSuccessfulScanTimestamp)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* GetApplicableUpdates)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate** result);
    HRESULT (STDMETHODCALLTYPE* GetMostRecentCompletedUpdates)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        INT32 count,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** result);
    HRESULT (STDMETHODCALLTYPE* GetMostRecentCompletedUpdatesAsync)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        INT32 count,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdateItem** operation);
    HRESULT (STDMETHODCALLTYPE* StartScan)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager* This,
        boolean userInitiated);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_add_ScanningStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_ScanningStateChanged(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_remove_ScanningStateChanged(This, token) \
    ((This)->lpVtbl->remove_ScanningStateChanged(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_add_WorkingStateChanged(This, handler, token) \
    ((This)->lpVtbl->add_WorkingStateChanged(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_remove_WorkingStateChanged(This, token) \
    ((This)->lpVtbl->remove_WorkingStateChanged(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_add_ProgressChanged(This, handler, token) \
    ((This)->lpVtbl->add_ProgressChanged(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_remove_ProgressChanged(This, token) \
    ((This)->lpVtbl->remove_ProgressChanged(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_add_AttentionRequiredReasonChanged(This, handler, token) \
    ((This)->lpVtbl->add_AttentionRequiredReasonChanged(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_remove_AttentionRequiredReasonChanged(This, token) \
    ((This)->lpVtbl->remove_AttentionRequiredReasonChanged(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_add_ActionCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ActionCompleted(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_remove_ActionCompleted(This, token) \
    ((This)->lpVtbl->remove_ActionCompleted(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_add_ScanCompleted(This, handler, token) \
    ((This)->lpVtbl->add_ScanCompleted(This, handler, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_remove_ScanCompleted(This, token) \
    ((This)->lpVtbl->remove_ScanCompleted(This, token))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_get_IsScanning(This, value) \
    ((This)->lpVtbl->get_IsScanning(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_get_IsWorking(This, value) \
    ((This)->lpVtbl->get_IsWorking(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_get_LastSuccessfulScanTimestamp(This, value) \
    ((This)->lpVtbl->get_LastSuccessfulScanTimestamp(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_GetApplicableUpdates(This, result) \
    ((This)->lpVtbl->GetApplicableUpdates(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_GetMostRecentCompletedUpdates(This, count, result) \
    ((This)->lpVtbl->GetMostRecentCompletedUpdates(This, count, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_GetMostRecentCompletedUpdatesAsync(This, count, operation) \
    ((This)->lpVtbl->GetMostRecentCompletedUpdatesAsync(This, count, operation))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_StartScan(This, userInitiated) \
    ((This)->lpVtbl->StartScan(This, userInitiated))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManager2
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManager2[] = L"Windows.Management.Update.IWindowsUpdateManager2";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetProvider)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        HSTRING id,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateProvider** result);
    HRESULT (STDMETHODCALLTYPE* get_ProviderIds)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        UINT32* valueLength,
        HSTRING** value);
    HRESULT (STDMETHODCALLTYPE* GetApplicableSoftwareUpdates)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsSoftwareUpdate** result);
    HRESULT (STDMETHODCALLTYPE* PerformScan)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* options,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsSoftwareUpdateScanResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2Vtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_GetProvider(This, id, result) \
    ((This)->lpVtbl->GetProvider(This, id, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_get_ProviderIds(This, valueLength, value) \
    ((This)->lpVtbl->get_ProviderIds(This, valueLength, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_GetApplicableSoftwareUpdates(This, result) \
    ((This)->lpVtbl->GetApplicableSoftwareUpdates(This, result))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_PerformScan(This, options, result) \
    ((This)->lpVtbl->PerformScan(This, options, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager2_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerFactory[] = L"Windows.Management.Update.IWindowsUpdateManagerFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory* This,
        HSTRING clientId,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_CreateInstance(This, clientId, value) \
    ((This)->lpVtbl->CreateInstance(This, clientId, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerFactory2
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManager
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerFactory2[] = L"Windows.Management.Update.IWindowsUpdateManagerFactory2";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2* This,
        HSTRING clientId,
        UINT32 providerIdFilterLength,
        HSTRING* providerIdFilter,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManager** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2Vtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_CreateInstance(This, clientId, providerIdFilterLength, providerIdFilter, value) \
    ((This)->lpVtbl->CreateInstance(This, clientId, providerIdFilterLength, providerIdFilter, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerFactory2_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerScanOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManagerScanOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerScanOptions[] = L"Windows.Management.Update.IWindowsUpdateManagerScanOptions";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsUserInitiated)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IsUserInitiated)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_AllowBypassThrottling)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_AllowBypassThrottling)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_PerformUpdateActions)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_PerformUpdateActions)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_get_IsUserInitiated(This, value) \
    ((This)->lpVtbl->get_IsUserInitiated(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_put_IsUserInitiated(This, value) \
    ((This)->lpVtbl->put_IsUserInitiated(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_get_AllowBypassThrottling(This, value) \
    ((This)->lpVtbl->get_AllowBypassThrottling(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_put_AllowBypassThrottling(This, value) \
    ((This)->lpVtbl->put_AllowBypassThrottling(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_get_PerformUpdateActions(This, value) \
    ((This)->lpVtbl->get_PerformUpdateActions(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_put_PerformUpdateActions(This, value) \
    ((This)->lpVtbl->put_PerformUpdateActions(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateManagerScanOptionsFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateManagerScanOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateManagerScanOptionsFactory[] = L"Windows.Management.Update.IWindowsUpdateManagerScanOptionsFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory* This,
        boolean isUserInitiated,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_CreateInstance(This, isUserInitiated, value) \
    ((This)->lpVtbl->CreateInstance(This, isUserInitiated, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateManagerScanOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateProgressChangedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateProgressChangedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Update)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdate** value);
    HRESULT (STDMETHODCALLTYPE* get_ActionProgress)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs* This,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateActionProgress** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_get_Update(This, value) \
    ((This)->lpVtbl->get_Update(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_get_ActionProgress(This, value) \
    ((This)->lpVtbl->get_ActionProgress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateProgressChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateRestartRequestOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateRestartRequestOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateRestartRequestOptions[] = L"Windows.Management.Update.IWindowsUpdateRestartRequestOptions";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Title)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Title)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_Description)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_MoreInfoUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* put_MoreInfoUrl)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* value);
    HRESULT (STDMETHODCALLTYPE* get_ComplianceDeadlineInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ComplianceDeadlineInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_ComplianceGracePeriodInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        INT32* value);
    HRESULT (STDMETHODCALLTYPE* put_ComplianceGracePeriodInDays)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        INT32 value);
    HRESULT (STDMETHODCALLTYPE* get_OrganizationName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* put_OrganizationName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        HSTRING value);
    HRESULT (STDMETHODCALLTYPE* get_OptOutOfAutoReboot)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_OptOutOfAutoReboot)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_Title(This, value) \
    ((This)->lpVtbl->get_Title(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_Title(This, value) \
    ((This)->lpVtbl->put_Title(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_Description(This, value) \
    ((This)->lpVtbl->put_Description(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_MoreInfoUrl(This, value) \
    ((This)->lpVtbl->get_MoreInfoUrl(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_MoreInfoUrl(This, value) \
    ((This)->lpVtbl->put_MoreInfoUrl(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_ComplianceDeadlineInDays(This, value) \
    ((This)->lpVtbl->get_ComplianceDeadlineInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_ComplianceDeadlineInDays(This, value) \
    ((This)->lpVtbl->put_ComplianceDeadlineInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_ComplianceGracePeriodInDays(This, value) \
    ((This)->lpVtbl->get_ComplianceGracePeriodInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_ComplianceGracePeriodInDays(This, value) \
    ((This)->lpVtbl->put_ComplianceGracePeriodInDays(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_OrganizationName(This, value) \
    ((This)->lpVtbl->get_OrganizationName(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_OrganizationName(This, value) \
    ((This)->lpVtbl->put_OrganizationName(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_get_OptOutOfAutoReboot(This, value) \
    ((This)->lpVtbl->get_OptOutOfAutoReboot(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_put_OptOutOfAutoReboot(This, value) \
    ((This)->lpVtbl->put_OptOutOfAutoReboot(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateRestartRequestOptions
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateRestartRequestOptionsFactory[] = L"Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory* This,
        HSTRING title,
        HSTRING description,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass* moreInfoUrl,
        INT32 complianceDeadlineInDays,
        INT32 complianceGracePeriodInDays,
        __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactoryVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_CreateInstance(This, title, description, moreInfoUrl, complianceDeadlineInDays, complianceGracePeriodInDays, value) \
    ((This)->lpVtbl->CreateInstance(This, title, description, moreInfoUrl, complianceDeadlineInDays, complianceGracePeriodInDays, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateRestartRequestOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.Management.Update.WindowsUpdateScanCompletedEventArgs
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_Management_Update_IWindowsUpdateScanCompletedEventArgs[] = L"Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs";
typedef struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProviderId)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Succeeded)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_Updates)(__x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs* This,
        __FIVectorView_1_Windows__CManagement__CUpdate__CWindowsUpdate** value);

    END_INTERFACE
} __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgsVtbl;

interface __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_get_ProviderId(This, value) \
    ((This)->lpVtbl->get_ProviderId(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_get_Succeeded(This, value) \
    ((This)->lpVtbl->get_Succeeded(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#define __x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_get_Updates(This, value) \
    ((This)->lpVtbl->get_Updates(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CManagement_CUpdate_CIWindowsUpdateScanCompletedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.PreviewBuildsManager
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Update.IPreviewBuildsManagerStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IPreviewBuildsManager ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Management_Update_PreviewBuildsManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_PreviewBuildsManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_PreviewBuildsManager[] = L"Windows.Management.Update.PreviewBuildsManager";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Management.Update.PreviewBuildsState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IPreviewBuildsState ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_Management_Update_PreviewBuildsState_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_PreviewBuildsState_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_PreviewBuildsState[] = L"Windows.Management.Update.PreviewBuildsState";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdate ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdate_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdate[] = L"Windows.Management.Update.WindowsSoftwareUpdate";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateActionInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateActionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateActionInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateActionInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateActionProgress ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionProgress_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionProgress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateActionProgress[] = L"Windows.Management.Update.WindowsSoftwareUpdateActionProgress";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateActionResultInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionResultInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateActionResultInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateActionResultInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateActionResultInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateAppPackageInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateAppPackageInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateAppPackageInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateAppPackageInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateAppPackageInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateApprovalInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateApprovalInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateApprovalInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateApprovalInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateApprovalInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateExecutionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateExecutionInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateExecutionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateExecutionInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateExecutionInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateLocalizationInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateLocalizationInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateLocalizationInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateLocalizationInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateLocalizationInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateOptionalActionInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalActionInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalActionInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateOptionalActionInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateOptionalActionInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfoFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateOptionalInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateOptionalInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateOptionalInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateOptionalInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProvider
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateProviderFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProvider ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProvider_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProvider_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProvider[] = L"Windows.Management.Update.WindowsSoftwareUpdateProvider";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResultFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProviderActionResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderActionResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderActionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProviderActionResult[] = L"Windows.Management.Update.WindowsSoftwareUpdateProviderActionResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProviderPayloadFileInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderPayloadFileInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderPayloadFileInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProviderPayloadFileInfo[] = L"Windows.Management.Update.WindowsSoftwareUpdateProviderPayloadFileInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateProviderStatus
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateProviderStatusFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateProviderStatus ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderStatus_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateProviderStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateProviderStatus[] = L"Windows.Management.Update.WindowsSoftwareUpdateProviderStatus";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateResultFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateResult[] = L"Windows.Management.Update.WindowsSoftwareUpdateResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateScanResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateScanResultFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateScanResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateScanResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateScanResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateScanResult[] = L"Windows.Management.Update.WindowsSoftwareUpdateScanResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsSoftwareUpdateVersion
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsSoftwareUpdateVersionFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsSoftwareUpdateVersion ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateVersion_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsSoftwareUpdateVersion_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsSoftwareUpdateVersion[] = L"Windows.Management.Update.WindowsSoftwareUpdateVersion";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsUpdate
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdate ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdate_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdate_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdate[] = L"Windows.Management.Update.WindowsUpdate";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateActionCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateActionCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateActionCompletedEventArgs[] = L"Windows.Management.Update.WindowsUpdateActionCompletedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateActionProgress
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateActionProgress ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionProgress_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionProgress_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateActionProgress[] = L"Windows.Management.Update.WindowsUpdateActionProgress";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateActionResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateActionResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateActionResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateActionResult[] = L"Windows.Management.Update.WindowsUpdateActionResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateAdministrator
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.Management.Update.IWindowsUpdateAdministratorStatics interface starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateAdministrator ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAdministrator_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAdministrator_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateAdministrator[] = L"Windows.Management.Update.WindowsUpdateAdministrator";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateApprovalData
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateApprovalData ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateApprovalData_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateApprovalData_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateApprovalData[] = L"Windows.Management.Update.WindowsUpdateApprovalData";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateAttentionRequiredInfo
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateAttentionRequiredInfo ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredInfo_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateAttentionRequiredInfo[] = L"Windows.Management.Update.WindowsUpdateAttentionRequiredInfo";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateAttentionRequiredReasonChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredReasonChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateAttentionRequiredReasonChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateAttentionRequiredReasonChangedEventArgs[] = L"Windows.Management.Update.WindowsUpdateAttentionRequiredReasonChangedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateGetAdministratorResult
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateGetAdministratorResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateGetAdministratorResult_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateGetAdministratorResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateGetAdministratorResult[] = L"Windows.Management.Update.WindowsUpdateGetAdministratorResult";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateItem
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateItem ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateItem_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateItem_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateItem[] = L"Windows.Management.Update.WindowsUpdateItem";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateManager
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateManagerFactory2 interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateManagerFactory interface starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateManager ** Default Interface **
 *    Windows.Management.Update.IWindowsUpdateManager2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManager_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManager_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateManager[] = L"Windows.Management.Update.WindowsUpdateManager";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateManagerScanOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 2.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateManagerScanOptionsFactory interface starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *   Type can be activated via RoActivateInstance starting with version 2.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateManagerScanOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManagerScanOptions_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateManagerScanOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateManagerScanOptions[] = L"Windows.Management.Update.WindowsUpdateManagerScanOptions";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateProgressChangedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateProgressChangedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateProgressChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateProgressChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateProgressChangedEventArgs[] = L"Windows.Management.Update.WindowsUpdateProgressChangedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateRestartRequestOptions
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.Management.Update.IWindowsUpdateRestartRequestOptionsFactory interface starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *   Type can be activated via RoActivateInstance starting with version 1.0 of the Windows.Management.Update.WindowsUpdateContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateRestartRequestOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateRestartRequestOptions_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateRestartRequestOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateRestartRequestOptions[] = L"Windows.Management.Update.WindowsUpdateRestartRequestOptions";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.Management.Update.WindowsUpdateScanCompletedEventArgs
 *
 * Introduced to Windows.Management.Update.WindowsUpdateContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.Management.Update.IWindowsUpdateScanCompletedEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_Management_Update_WindowsUpdateScanCompletedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_Management_Update_WindowsUpdateScanCompletedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_Management_Update_WindowsUpdateScanCompletedEventArgs[] = L"Windows.Management.Update.WindowsUpdateScanCompletedEventArgs";
#endif
#endif // WINDOWS_MANAGEMENT_UPDATE_WINDOWSUPDATECONTRACT_VERSION >= 0x10000

#endif // defined(__cplusplus)
#pragma pop_macro("MIDL_CONST_ID")
// Restore the original value of the 'DEPRECATED' macro
#pragma pop_macro("DEPRECATED")

#ifdef __clang__
#pragma clang diagnostic pop // deprecated-declarations
#else
#pragma warning(pop)
#endif
#endif // __windows2Emanagement2Eupdate_p_h__

#endif // __windows2Emanagement2Eupdate_h__
