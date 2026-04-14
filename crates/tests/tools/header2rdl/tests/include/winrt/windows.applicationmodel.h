
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
#ifndef __windows2Eapplicationmodel_h__
#define __windows2Eapplicationmodel_h__
#ifndef __windows2Eapplicationmodel_p_h__
#define __windows2Eapplicationmodel_p_h__


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
#if !defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION 0x20000
#endif // defined(WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION 0x30000
#endif // defined(WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_ACTIVATIONCAMERASETTINGSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_CONTACTACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)
#define WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION 0x10000
#endif // defined(WINDOWS_APPLICATIONMODEL_ACTIVATION_WEBUISEARCHACTIVATEDEVENTSCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)
#define WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION 0x40000
#endif // defined(WINDOWS_FOUNDATION_FOUNDATIONCONTRACT_VERSION)

#if !defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)
#define WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION 0x130000
#endif // defined(WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION)

#if !defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)
#define WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION 0x70000
#endif // defined(WINDOWS_SYSTEM_SYSTEMMANAGEMENTCONTRACT_VERSION)

#endif // defined(SPECIFIC_API_CONTRACT_DEFINITIONS)


// Header files for imported files
#include "inspectable.h"
#include "AsyncInfo.h"
#include "EventToken.h"
#include "windowscontracts.h"
#include "Windows.Foundation.h"
#include "Windows.ApplicationModel.Activation.h"
#include "Windows.ApplicationModel.Core.h"
#include "Windows.Storage.h"
#include "Windows.Storage.Streams.h"
#include "Windows.System.h"
// Importing Collections header
#include <windows.foundation.collections.h>

#if defined(__cplusplus) && !defined(CINTERFACE)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppDisplayInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo ABI::Windows::ApplicationModel::IAppDisplayInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo ABI::Windows::ApplicationModel::IAppInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2 ABI::Windows::ApplicationModel::IAppInfo2

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo3;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3 ABI::Windows::ApplicationModel::IAppInfo3

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfo4;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4 ABI::Windows::ApplicationModel::IAppInfo4

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInfoStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics ABI::Windows::ApplicationModel::IAppInfoStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInstallerInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo ABI::Windows::ApplicationModel::IAppInstallerInfo

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInstallerInfo2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2 ABI::Windows::ApplicationModel::IAppInstallerInfo2

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstance_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInstance;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInstance ABI::Windows::ApplicationModel::IAppInstance

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IAppInstanceStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics ABI::Windows::ApplicationModel::IAppInstanceStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IDesignModeStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics ABI::Windows::ApplicationModel::IDesignModeStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IDesignModeStatics2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2 ABI::Windows::ApplicationModel::IDesignModeStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IEnteredBackgroundEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs ABI::Windows::ApplicationModel::IEnteredBackgroundEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IFindRelatedPackagesOptions;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions ABI::Windows::ApplicationModel::IFindRelatedPackagesOptions

#endif // ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IFindRelatedPackagesOptionsFactory;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory ABI::Windows::ApplicationModel::IFindRelatedPackagesOptionsFactory

#endif // ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IFullTrustProcessLaunchResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult ABI::Windows::ApplicationModel::IFullTrustProcessLaunchResult

#endif // ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IFullTrustProcessLauncherStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics ABI::Windows::ApplicationModel::IFullTrustProcessLauncherStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IFullTrustProcessLauncherStatics2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2 ABI::Windows::ApplicationModel::IFullTrustProcessLauncherStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ILeavingBackgroundEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs ABI::Windows::ApplicationModel::ILeavingBackgroundEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ILimitedAccessFeatureRequestResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult ABI::Windows::ApplicationModel::ILimitedAccessFeatureRequestResult

#endif // ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ILimitedAccessFeaturesStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics ABI::Windows::ApplicationModel::ILimitedAccessFeaturesStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage ABI::Windows::ApplicationModel::IPackage

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage2 ABI::Windows::ApplicationModel::IPackage2

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage3;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage3 ABI::Windows::ApplicationModel::IPackage3

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage4;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage4 ABI::Windows::ApplicationModel::IPackage4

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage5_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage5;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage5 ABI::Windows::ApplicationModel::IPackage5

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage6_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage6_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage6;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage6 ABI::Windows::ApplicationModel::IPackage6

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage7_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage7_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage7;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage7 ABI::Windows::ApplicationModel::IPackage7

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage8_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage8_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage8;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage8 ABI::Windows::ApplicationModel::IPackage8

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage9_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage9_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackage9;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackage9 ABI::Windows::ApplicationModel::IPackage9

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalog;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog ABI::Windows::ApplicationModel::IPackageCatalog

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalog2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2 ABI::Windows::ApplicationModel::IPackageCatalog2

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalog3;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3 ABI::Windows::ApplicationModel::IPackageCatalog3

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalog4;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4 ABI::Windows::ApplicationModel::IPackageCatalog4

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalogAddOptionalPackageResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult ABI::Windows::ApplicationModel::IPackageCatalogAddOptionalPackageResult

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalogAddResourcePackageResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult ABI::Windows::ApplicationModel::IPackageCatalogAddResourcePackageResult

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalogRemoveOptionalPackagesResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult ABI::Windows::ApplicationModel::IPackageCatalogRemoveOptionalPackagesResult

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalogRemoveResourcePackagesResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult ABI::Windows::ApplicationModel::IPackageCatalogRemoveResourcePackagesResult

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalogStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics ABI::Windows::ApplicationModel::IPackageCatalogStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageCatalogStatics2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2 ABI::Windows::ApplicationModel::IPackageCatalogStatics2

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageContentGroup;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup ABI::Windows::ApplicationModel::IPackageContentGroup

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageContentGroupStagingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs ABI::Windows::ApplicationModel::IPackageContentGroupStagingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageContentGroupStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics ABI::Windows::ApplicationModel::IPackageContentGroupStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageId_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageId_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageId;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageId ABI::Windows::ApplicationModel::IPackageId

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageIdWithMetadata;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata ABI::Windows::ApplicationModel::IPackageIdWithMetadata

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageInstallingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs ABI::Windows::ApplicationModel::IPackageInstallingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageStagingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs ABI::Windows::ApplicationModel::IPackageStagingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics ABI::Windows::ApplicationModel::IPackageStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageStatus;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus ABI::Windows::ApplicationModel::IPackageStatus

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageStatus2;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2 ABI::Windows::ApplicationModel::IPackageStatus2

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageStatusChangedEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs ABI::Windows::ApplicationModel::IPackageStatusChangedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageUninstallingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs ABI::Windows::ApplicationModel::IPackageUninstallingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageUpdateAvailabilityResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult ABI::Windows::ApplicationModel::IPackageUpdateAvailabilityResult

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageUpdatingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs ABI::Windows::ApplicationModel::IPackageUpdatingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IPackageWithMetadata;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata ABI::Windows::ApplicationModel::IPackageWithMetadata

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIStartupTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTask_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IStartupTask;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIStartupTask ABI::Windows::ApplicationModel::IStartupTask

#endif // ____x_ABI_CWindows_CApplicationModel_CIStartupTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface IStartupTaskStatics;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics ABI::Windows::ApplicationModel::IStartupTaskStatics

#endif // ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ISuspendingDeferral;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral ABI::Windows::ApplicationModel::ISuspendingDeferral

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ISuspendingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs ABI::Windows::ApplicationModel::ISuspendingEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            interface ISuspendingOperation;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation ABI::Windows::ApplicationModel::ISuspendingOperation

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__

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
        namespace ApplicationModel {
            class FullTrustProcessLaunchResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("e37e376e-965e-5fff-a66f-18309b981ea5"))
IAsyncOperation<ABI::Windows::ApplicationModel::FullTrustProcessLaunchResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::FullTrustProcessLaunchResult*, ABI::Windows::ApplicationModel::IFullTrustProcessLaunchResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.FullTrustProcessLaunchResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::FullTrustProcessLaunchResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_USE */

#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c938b875-ac86-5946-a7e0-f2a147601ada"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::FullTrustProcessLaunchResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::FullTrustProcessLaunchResult*, ABI::Windows::ApplicationModel::IFullTrustProcessLaunchResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.FullTrustProcessLaunchResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::FullTrustProcessLaunchResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_USE */

#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageCatalogAddOptionalPackageResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("59b2497f-86eb-542f-bea6-1be53e93e13d"))
IAsyncOperation<ABI::Windows::ApplicationModel::PackageCatalogAddOptionalPackageResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogAddOptionalPackageResult*, ABI::Windows::ApplicationModel::IPackageCatalogAddOptionalPackageResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::PackageCatalogAddOptionalPackageResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("6c5f9b5d-3c24-5087-ae15-6ab4942c4639"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogAddOptionalPackageResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogAddOptionalPackageResult*, ABI::Windows::ApplicationModel::IPackageCatalogAddOptionalPackageResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogAddOptionalPackageResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageCatalogRemoveOptionalPackagesResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cfc179aa-fb98-54ef-8ea8-64499347b7f7"))
IAsyncOperation<ABI::Windows::ApplicationModel::PackageCatalogRemoveOptionalPackagesResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogRemoveOptionalPackagesResult*, ABI::Windows::ApplicationModel::IPackageCatalogRemoveOptionalPackagesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::PackageCatalogRemoveOptionalPackagesResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("f168612c-6882-5c8c-a464-7ea25e269876"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogRemoveOptionalPackagesResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogRemoveOptionalPackagesResult*, ABI::Windows::ApplicationModel::IPackageCatalogRemoveOptionalPackagesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogRemoveOptionalPackagesResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageCatalogRemoveResourcePackagesResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("342ee07c-8d6e-53a4-b746-c5b74bb61f6d"))
IAsyncOperation<ABI::Windows::ApplicationModel::PackageCatalogRemoveResourcePackagesResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogRemoveResourcePackagesResult*, ABI::Windows::ApplicationModel::IPackageCatalogRemoveResourcePackagesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::PackageCatalogRemoveResourcePackagesResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("142b78d7-5cc3-5b40-a58d-43a582ca83ea"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogRemoveResourcePackagesResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogRemoveResourcePackagesResult*, ABI::Windows::ApplicationModel::IPackageCatalogRemoveResourcePackagesResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogRemoveResourcePackagesResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageContentGroup;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bbd292e3-db9f-5802-a488-40f156332c04"))
IAsyncOperation<ABI::Windows::ApplicationModel::PackageContentGroup*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroup*, ABI::Windows::ApplicationModel::IPackageContentGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.PackageContentGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::PackageContentGroup*> __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("2253dc38-9a1a-5364-9a3b-03a7da615499"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageContentGroup*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroup*, ABI::Windows::ApplicationModel::IPackageContentGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.PackageContentGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageContentGroup*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageUpdateAvailabilityResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("010bd015-43ef-576c-be1e-bc38c5b6b66b"))
IAsyncOperation<ABI::Windows::ApplicationModel::PackageUpdateAvailabilityResult*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageUpdateAvailabilityResult*, ABI::Windows::ApplicationModel::IPackageUpdateAvailabilityResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.PackageUpdateAvailabilityResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::PackageUpdateAvailabilityResult*> __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("4020c034-c762-5152-beef-f03471e885b9"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageUpdateAvailabilityResult*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageUpdateAvailabilityResult*, ABI::Windows::ApplicationModel::IPackageUpdateAvailabilityResult*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.PackageUpdateAvailabilityResult>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::PackageUpdateAvailabilityResult*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class StartupTask;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("cbec7a4e-a046-5330-873d-0fce228792fa"))
IAsyncOperation<ABI::Windows::ApplicationModel::StartupTask*> : IAsyncOperation_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::StartupTask*, ABI::Windows::ApplicationModel::IStartupTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.StartupTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<ABI::Windows::ApplicationModel::StartupTask*> __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("741f7697-2452-5c80-83c6-3b6f822b904c"))
IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::StartupTask*> : IAsyncOperationCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::StartupTask*, ABI::Windows::ApplicationModel::IStartupTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.StartupTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<ABI::Windows::ApplicationModel::StartupTask*> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum StartupTaskState : int StartupTaskState;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_USE
#define DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("5239a934-80e2-518f-b819-1f316f379a3f"))
IAsyncOperation<enum ABI::Windows::ApplicationModel::StartupTaskState> : IAsyncOperation_impl<enum ABI::Windows::ApplicationModel::StartupTaskState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.ApplicationModel.StartupTaskState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<enum ABI::Windows::ApplicationModel::StartupTaskState> __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_t;
#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState ABI::Windows::Foundation::__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_USE
#define DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("70a0bf67-19e8-5a86-a32e-3c9863825a04"))
IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::StartupTaskState> : IAsyncOperationCompletedHandler_impl<enum ABI::Windows::ApplicationModel::StartupTaskState>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.ApplicationModel.StartupTaskState>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<enum ABI::Windows::ApplicationModel::StartupTaskState> __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_t;
#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                class AppListEntry;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Core {
                interface IAppListEntry;
            } /* Core */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry ABI::Windows::ApplicationModel::Core::IAppListEntry

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("b93e2028-50bc-599e-b3d9-427b61d26c01"))
IIterator<ABI::Windows::ApplicationModel::Core::AppListEntry*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::AppListEntry*, ABI::Windows::ApplicationModel::Core::IAppListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Core.AppListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Core::AppListEntry*> __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_t;
#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("86f4d4ef-d8fd-5fb5-807c-72da8fc9e544"))
IIterable<ABI::Windows::ApplicationModel::Core::AppListEntry*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::AppListEntry*, ABI::Windows::ApplicationModel::Core::IAppListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Core.AppListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Core::AppListEntry*> __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_t;
#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("920c8b92-d5ef-5899-8776-2ad97aca6e1d"))
IVectorView<ABI::Windows::ApplicationModel::Core::AppListEntry*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Core::AppListEntry*, ABI::Windows::ApplicationModel::Core::IAppListEntry*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Core.AppListEntry>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Core::AppListEntry*> __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_t;
#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("d3bcf8a0-3538-5dae-98d7-1f2ab88c3f01"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Core.AppListEntry>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("51c74372-9452-57ce-9270-762009fbfe4d"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Core.AppListEntry>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("14653598-9065-508e-b37b-44eb28d51cb7"))
IIterator<ABI::Windows::ApplicationModel::StartupTask*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::StartupTask*, ABI::Windows::ApplicationModel::IStartupTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.StartupTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::StartupTask*> __FIIterator_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIIterator_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CStartupTask_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("61885ead-bf9e-5e9f-af04-6296b336930b"))
IIterable<ABI::Windows::ApplicationModel::StartupTask*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::StartupTask*, ABI::Windows::ApplicationModel::IStartupTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.StartupTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::StartupTask*> __FIIterable_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIIterable_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CStartupTask_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("ea3b1b7a-5071-5986-88fc-912dbbf845fb"))
IVectorView<ABI::Windows::ApplicationModel::StartupTask*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::StartupTask*, ABI::Windows::ApplicationModel::IStartupTask*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.StartupTask>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::StartupTask*> __FIVectorView_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CStartupTask_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("16543165-5b65-5683-8a02-c385659158bd"))
IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStartupTask*> : IAsyncOperation_impl<__FIVectorView_1_Windows__CApplicationModel__CStartupTask*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.StartupTask>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVectorView_1_Windows__CApplicationModel__CStartupTask*> __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("15d40795-41f9-50d7-a39e-5390981af651"))
IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStartupTask*> : IAsyncOperationCompletedHandler_impl<__FIVectorView_1_Windows__CApplicationModel__CStartupTask*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.StartupTask>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVectorView_1_Windows__CApplicationModel__CStartupTask*> __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_t;
#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_USE */

#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("5f23d323-28f5-560f-a40e-6f3827f82e9f"))
IIterator<ABI::Windows::ApplicationModel::PackageContentGroup*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroup*, ABI::Windows::ApplicationModel::IPackageContentGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.PackageContentGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::PackageContentGroup*> __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d7dd1456-4805-5768-a25d-99641b096491"))
IIterable<ABI::Windows::ApplicationModel::PackageContentGroup*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroup*, ABI::Windows::ApplicationModel::IPackageContentGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.PackageContentGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::PackageContentGroup*> __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("deae10f1-0546-5e61-bd3f-e05da30078ac"))
IVectorView<ABI::Windows::ApplicationModel::PackageContentGroup*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroup*, ABI::Windows::ApplicationModel::IPackageContentGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.PackageContentGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::PackageContentGroup*> __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("29adc699-5848-5a98-a516-23feb0fa2c4b"))
IVector<ABI::Windows::ApplicationModel::PackageContentGroup*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroup*, ABI::Windows::ApplicationModel::IPackageContentGroup*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.PackageContentGroup>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::PackageContentGroup*> __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("929e3c29-bf29-5594-bc63-67db43a539ea"))
IAsyncOperation<__FIVector_1_Windows__CApplicationModel__CPackageContentGroup*> : IAsyncOperation_impl<__FIVector_1_Windows__CApplicationModel__CPackageContentGroup*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperation`1<Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.PackageContentGroup>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperation<__FIVector_1_Windows__CApplicationModel__CPackageContentGroup*> __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE
#define DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("52465bf3-3ca6-5681-a7b4-91847757b5fd"))
IAsyncOperationCompletedHandler<__FIVector_1_Windows__CApplicationModel__CPackageContentGroup*> : IAsyncOperationCompletedHandler_impl<__FIVector_1_Windows__CApplicationModel__CPackageContentGroup*>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationCompletedHandler`1<Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.PackageContentGroup>>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationCompletedHandler<__FIVector_1_Windows__CApplicationModel__CPackageContentGroup*> __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_t;
#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup ABI::Windows::Foundation::__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageCatalogAddResourcePackageResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef struct PackageInstallProgress PackageInstallProgress;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE
#define DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("76fd3482-764c-5806-8758-035aefb6a548"))
IAsyncOperationWithProgressCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, struct ABI::Windows::ApplicationModel::PackageInstallProgress> : IAsyncOperationWithProgressCompletedHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, ABI::Windows::ApplicationModel::IPackageCatalogAddResourcePackageResult*>, struct ABI::Windows::ApplicationModel::PackageInstallProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationWithProgressCompletedHandler`2<Windows.ApplicationModel.PackageCatalogAddResourcePackageResult, Windows.ApplicationModel.PackageInstallProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgressCompletedHandler<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, struct ABI::Windows::ApplicationModel::PackageInstallProgress> __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_t;
#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE
#define DEF___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b147e038-cc1f-567f-b1f1-64d5d4003309"))
IAsyncOperationWithProgress<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, struct ABI::Windows::ApplicationModel::PackageInstallProgress> : IAsyncOperationWithProgress_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, ABI::Windows::ApplicationModel::IPackageCatalogAddResourcePackageResult*>, struct ABI::Windows::ApplicationModel::PackageInstallProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.IAsyncOperationWithProgress`2<Windows.ApplicationModel.PackageCatalogAddResourcePackageResult, Windows.ApplicationModel.PackageInstallProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationWithProgress<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, struct ABI::Windows::ApplicationModel::PackageInstallProgress> __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_t;
#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress ABI::Windows::Foundation::__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE
#define DEF___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a58762e7-fcc2-5b31-945e-71c6c9bc516e"))
IAsyncOperationProgressHandler<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, struct ABI::Windows::ApplicationModel::PackageInstallProgress> : IAsyncOperationProgressHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, ABI::Windows::ApplicationModel::IPackageCatalogAddResourcePackageResult*>, struct ABI::Windows::ApplicationModel::PackageInstallProgress>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.AsyncOperationProgressHandler`2<Windows.ApplicationModel.PackageCatalogAddResourcePackageResult, Windows.ApplicationModel.PackageInstallProgress>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IAsyncOperationProgressHandler<ABI::Windows::ApplicationModel::PackageCatalogAddResourcePackageResult*, struct ABI::Windows::ApplicationModel::PackageInstallProgress> __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_t;
#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress ABI::Windows::Foundation::__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000


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
        namespace ApplicationModel {
            class AppInstance;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CAppInstance_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CAppInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("14c94628-7938-56d5-895a-8fc52ed3bb7e"))
IIterator<ABI::Windows::ApplicationModel::AppInstance*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInstance*, ABI::Windows::ApplicationModel::IAppInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.AppInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::AppInstance*> __FIIterator_1_Windows__CApplicationModel__CAppInstance_t;
#define __FIIterator_1_Windows__CApplicationModel__CAppInstance ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CAppInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CAppInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CAppInstance_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CAppInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("2c28b4c9-6378-5382-a1a6-79bd6eed967a"))
IIterable<ABI::Windows::ApplicationModel::AppInstance*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInstance*, ABI::Windows::ApplicationModel::IAppInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.AppInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::AppInstance*> __FIIterable_1_Windows__CApplicationModel__CAppInstance_t;
#define __FIIterable_1_Windows__CApplicationModel__CAppInstance ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CAppInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CAppInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class Package;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterator_1_Windows__CApplicationModel__CPackage_USE
#define DEF___FIIterator_1_Windows__CApplicationModel__CPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0217f069-025c-5ee6-a87f-e782e3b623ae"))
IIterator<ABI::Windows::ApplicationModel::Package*> : IIterator_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Package*, ABI::Windows::ApplicationModel::IPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterator`1<Windows.ApplicationModel.Package>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterator<ABI::Windows::ApplicationModel::Package*> __FIIterator_1_Windows__CApplicationModel__CPackage_t;
#define __FIIterator_1_Windows__CApplicationModel__CPackage ABI::Windows::Foundation::Collections::__FIIterator_1_Windows__CApplicationModel__CPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterator_1_Windows__CApplicationModel__CPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIIterable_1_Windows__CApplicationModel__CPackage_USE
#define DEF___FIIterable_1_Windows__CApplicationModel__CPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("69ad6aa7-0c49-5f27-a5eb-ef4d59467b6d"))
IIterable<ABI::Windows::ApplicationModel::Package*> : IIterable_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Package*, ABI::Windows::ApplicationModel::IPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IIterable`1<Windows.ApplicationModel.Package>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IIterable<ABI::Windows::ApplicationModel::Package*> __FIIterable_1_Windows__CApplicationModel__CPackage_t;
#define __FIIterable_1_Windows__CApplicationModel__CPackage ABI::Windows::Foundation::Collections::__FIIterable_1_Windows__CApplicationModel__CPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIIterable_1_Windows__CApplicationModel__CPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CAppInstance_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CAppInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d1864e26-b62b-5085-8e93-60cbb6eb20d1"))
IVectorView<ABI::Windows::ApplicationModel::AppInstance*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInstance*, ABI::Windows::ApplicationModel::IAppInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.AppInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::AppInstance*> __FIVectorView_1_Windows__CApplicationModel__CAppInstance_t;
#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CAppInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CAppInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVectorView_1_Windows__CApplicationModel__CPackage_USE
#define DEF___FIVectorView_1_Windows__CApplicationModel__CPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("0263c4d4-195c-5dc5-a7ca-6806ceca420b"))
IVectorView<ABI::Windows::ApplicationModel::Package*> : IVectorView_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Package*, ABI::Windows::ApplicationModel::IPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVectorView`1<Windows.ApplicationModel.Package>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVectorView<ABI::Windows::ApplicationModel::Package*> __FIVectorView_1_Windows__CApplicationModel__CPackage_t;
#define __FIVectorView_1_Windows__CApplicationModel__CPackage ABI::Windows::Foundation::Collections::__FIVectorView_1_Windows__CApplicationModel__CPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVectorView_1_Windows__CApplicationModel__CPackage_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CAppInstance_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CAppInstance_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("7ff85c5e-7752-5ef0-bf29-020648c199e4"))
IVector<ABI::Windows::ApplicationModel::AppInstance*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::AppInstance*, ABI::Windows::ApplicationModel::IAppInstance*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.AppInstance>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::AppInstance*> __FIVector_1_Windows__CApplicationModel__CAppInstance_t;
#define __FIVector_1_Windows__CApplicationModel__CAppInstance ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CAppInstance_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CAppInstance_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#ifndef DEF___FIVector_1_Windows__CApplicationModel__CPackage_USE
#define DEF___FIVector_1_Windows__CApplicationModel__CPackage_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation { namespace Collections {
template <>
struct __declspec(uuid("d1bb509e-6989-5c69-b1ff-d1702fe8aca3"))
IVector<ABI::Windows::ApplicationModel::Package*> : IVector_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::Package*, ABI::Windows::ApplicationModel::IPackage*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.Collections.IVector`1<Windows.ApplicationModel.Package>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef IVector<ABI::Windows::ApplicationModel::Package*> __FIVector_1_Windows__CApplicationModel__CPackage_t;
#define __FIVector_1_Windows__CApplicationModel__CPackage ABI::Windows::Foundation::Collections::__FIVector_1_Windows__CApplicationModel__CPackage_t
/* Collections */ } /* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FIVector_1_Windows__CApplicationModel__CPackage_USE */

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
        namespace ApplicationModel {
            class PackageCatalog;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageContentGroupStagingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("df206861-9d30-5739-88b2-97e0717cc5f0"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageContentGroupStagingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::IPackageCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageContentGroupStagingEventArgs*, ABI::Windows::ApplicationModel::IPackageContentGroupStagingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageCatalog, Windows.ApplicationModel.PackageContentGroupStagingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageContentGroupStagingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageInstallingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("a8a900c6-da0b-5bcc-a71a-be0b9265d87a"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageInstallingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::IPackageCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageInstallingEventArgs*, ABI::Windows::ApplicationModel::IPackageInstallingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageCatalog, Windows.ApplicationModel.PackageInstallingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageInstallingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageStagingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("1726f52d-2b8c-524a-98c6-f2cf0893c0f2"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageStagingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::IPackageCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageStagingEventArgs*, ABI::Windows::ApplicationModel::IPackageStagingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageCatalog, Windows.ApplicationModel.PackageStagingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageStagingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageStatusChangedEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("b32d7d63-cd0e-5c2e-a251-fb8d290824e4"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageStatusChangedEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::IPackageCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageStatusChangedEventArgs*, ABI::Windows::ApplicationModel::IPackageStatusChangedEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageCatalog, Windows.ApplicationModel.PackageStatusChangedEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageStatusChangedEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageUninstallingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("bd636cf1-541f-53ea-8efc-e1604a395b1a"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageUninstallingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::IPackageCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageUninstallingEventArgs*, ABI::Windows::ApplicationModel::IPackageUninstallingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageCatalog, Windows.ApplicationModel.PackageUninstallingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageUninstallingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageUpdatingEventArgs;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_USE
#define DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_USE
#if !defined(RO_NO_TEMPLATE_NAME)
namespace ABI { namespace Windows { namespace Foundation {
template <>
struct __declspec(uuid("c23e15f6-c618-522a-82ab-4fab36665ce5"))
ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageUpdatingEventArgs*> : ITypedEventHandler_impl<ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::IPackageCatalog*>, ABI::Windows::Foundation::Internal::AggregateType<ABI::Windows::ApplicationModel::PackageUpdatingEventArgs*, ABI::Windows::ApplicationModel::IPackageUpdatingEventArgs*>>
{
    static const wchar_t* z_get_rc_name_impl()
    {
        return L"Windows.Foundation.TypedEventHandler`2<Windows.ApplicationModel.PackageCatalog, Windows.ApplicationModel.PackageUpdatingEventArgs>";
    }
};
// Define a typedef for the parameterized interface specialization's mangled name.
// This allows code which uses the mangled name for the parameterized interface to access the
// correct parameterized interface specialization.
typedef ITypedEventHandler<ABI::Windows::ApplicationModel::PackageCatalog*, ABI::Windows::ApplicationModel::PackageUpdatingEventArgs*> __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_t;
#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs ABI::Windows::Foundation::__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_t
/* Foundation */ } /* Windows */ } /* ABI */ }

#endif // !defined(RO_NO_TEMPLATE_NAME)
#endif /* DEF___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_USE */

#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            namespace Activation {
                interface IActivatedEventArgs;
            } /* Activation */
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

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
            typedef struct Size Size;
        } /* Foundation */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace Storage {
            class StorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace Storage {
            interface IStorageFolder;
        } /* Storage */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CStorage_CIStorageFolder ABI::Windows::Storage::IStorageFolder

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

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
        namespace System {
            typedef enum ProcessorArchitecture : int ProcessorArchitecture;
        } /* System */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace System {
            class User;
        } /* System */
    } /* Windows */
} /* ABI */

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
namespace ABI {
    namespace Windows {
        namespace System {
            interface IUser;
        } /* System */
    } /* Windows */
} /* ABI */
#define __x_ABI_CWindows_CSystem_CIUser ABI::Windows::System::IUser

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum AddResourcePackageOptions : unsigned int AddResourcePackageOptions;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum AppExecutionContext : int AppExecutionContext;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum AppInstallerPolicySource : int AppInstallerPolicySource;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum FullTrustLaunchResult : int FullTrustLaunchResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum LimitedAccessFeatureStatus : int LimitedAccessFeatureStatus;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum PackageContentGroupState : int PackageContentGroupState;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum PackageRelationship : int PackageRelationship;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum PackageSignatureKind : int PackageSignatureKind;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef enum PackageUpdateAvailability : int PackageUpdateAvailability;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            typedef struct PackageVersion PackageVersion;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppDisplayInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class AppInstallerInfo;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class FindRelatedPackagesOptions;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class LimitedAccessFeatureRequestResult;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageId;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class PackageStatus;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class SuspendingDeferral;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            class SuspendingOperation;
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

/*
 *
 * Struct Windows.ApplicationModel.AddResourcePackageOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum AddResourcePackageOptions : unsigned int
            {
                AddResourcePackageOptions_None = 0,
                AddResourcePackageOptions_ForceTargetAppShutdown = 0x1,
                AddResourcePackageOptions_ApplyUpdateIfAvailable = 0x2,
            };

            DEFINE_ENUM_FLAG_OPERATORS(AddResourcePackageOptions)
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.AppExecutionContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum AppExecutionContext : int
            {
                AppExecutionContext_Unknown = 0,
                AppExecutionContext_Host = 1,
                AppExecutionContext_Guest = 2,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Struct Windows.ApplicationModel.AppInstallerPolicySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum AppInstallerPolicySource : int
            {
                AppInstallerPolicySource_Default = 0,
                AppInstallerPolicySource_System = 1,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.FullTrustLaunchResult
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum FullTrustLaunchResult : int
            {
                FullTrustLaunchResult_Success = 0,
                FullTrustLaunchResult_AccessDenied = 1,
                FullTrustLaunchResult_FileNotFound = 2,
                FullTrustLaunchResult_Unknown = 3,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.ApplicationModel.LimitedAccessFeatureStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum LimitedAccessFeatureStatus : int
            {
                LimitedAccessFeatureStatus_Unavailable = 0,
                LimitedAccessFeatureStatus_Available = 1,
                LimitedAccessFeatureStatus_AvailableWithoutToken = 2,
                LimitedAccessFeatureStatus_Unknown = 3,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.PackageContentGroupState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum PackageContentGroupState : int
            {
                PackageContentGroupState_NotStaged = 0,
                PackageContentGroupState_Queued = 1,
                PackageContentGroupState_Staging = 2,
                PackageContentGroupState_Staged = 3,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.PackageRelationship
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum PackageRelationship : int
            {
                PackageRelationship_Dependencies = 0,
                PackageRelationship_Dependents = 1,
                PackageRelationship_All = 2,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.ApplicationModel.PackageSignatureKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum PackageSignatureKind : int
            {
                PackageSignatureKind_None = 0,
                PackageSignatureKind_Developer = 1,
                PackageSignatureKind_Enterprise = 2,
                PackageSignatureKind_Store = 3,
                PackageSignatureKind_System = 4,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.PackageUpdateAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum PackageUpdateAvailability : int
            {
                PackageUpdateAvailability_Unknown = 0,
                PackageUpdateAvailability_NoUpdates = 1,
                PackageUpdateAvailability_Available = 2,
                PackageUpdateAvailability_Required = 3,
                PackageUpdateAvailability_Error = 4,
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.StartupTaskState
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            enum StartupTaskState : int
            {
                StartupTaskState_Disabled = 0,
                StartupTaskState_DisabledByUser = 1,
                StartupTaskState_Enabled = 2,
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x20000
                StartupTaskState_DisabledByPolicy = 3,
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x20000
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x30000
                StartupTaskState_EnabledByPolicy = 4,
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x30000
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.PackageInstallProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            struct PackageInstallProgress
            {
                UINT32 PercentComplete;
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.PackageVersion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            struct PackageVersion
            {
                UINT16 Major;
                UINT16 Minor;
                UINT16 Build;
                UINT16 Revision;
            };
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IAppDisplayInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppDisplayInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppDisplayInfo[] = L"Windows.ApplicationModel.IAppDisplayInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("1aeb1103-e4d4-41aa-a4f6-c4a276e79eac")
            IAppDisplayInfo : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Description(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetLogo(
                    ABI::Windows::Foundation::Size size,
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppDisplayInfo = __uuidof(IAppDisplayInfo);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo[] = L"Windows.ApplicationModel.IAppInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("cf7f59b3-6a09-4de8-a6c0-5792d56880d1")
            IAppInfo : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Id(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AppUserModelId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DisplayInfo(
                    ABI::Windows::ApplicationModel::IAppDisplayInfo** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PackageFamilyName(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInfo = __uuidof(IAppInfo);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo2[] = L"Windows.ApplicationModel.IAppInfo2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("be4b1f5a-2098-431b-bd25-b30878748d47")
            IAppInfo2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInfo2 = __uuidof(IAppInfo2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo3[] = L"Windows.ApplicationModel.IAppInfo3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("09a78e46-93a4-46de-9397-0843b57115ea")
            IAppInfo3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ExecutionContext(
                    ABI::Windows::ApplicationModel::AppExecutionContext* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInfo3 = __uuidof(IAppInfo3);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo4[] = L"Windows.ApplicationModel.IAppInfo4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("2f34bdeb-1609-4554-9f33-12e1e803e0d4")
            IAppInfo4 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_SupportedFileExtensions(
                    UINT32* valueLength,
                    HSTRING** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInfo4 = __uuidof(IAppInfo4);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfoStatics[] = L"Windows.ApplicationModel.IAppInfoStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("cf1f782a-e48b-4f0c-9b0b-79c3f8957dd7")
            IAppInfoStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Current(
                    ABI::Windows::ApplicationModel::IAppInfo** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFromAppUserModelId(
                    HSTRING appUserModelId,
                    ABI::Windows::ApplicationModel::IAppInfo** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetFromAppUserModelIdForUser(
                    ABI::Windows::System::IUser* user,
                    HSTRING appUserModelId,
                    ABI::Windows::ApplicationModel::IAppInfo** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInfoStatics = __uuidof(IAppInfoStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstallerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstallerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstallerInfo[] = L"Windows.ApplicationModel.IAppInstallerInfo";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("29ab2ac0-d4f6-42a3-adcd-d6583c659508")
            IAppInstallerInfo : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Uri(
                    ABI::Windows::Foundation::IUriRuntimeClass** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInstallerInfo = __uuidof(IAppInstallerInfo);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstallerInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstallerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstallerInfo2[] = L"Windows.ApplicationModel.IAppInstallerInfo2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("d20f1388-8256-597c-8511-c84ec50d5e2b")
            IAppInstallerInfo2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_OnLaunch(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_HoursBetweenUpdateChecks(
                    UINT32* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ShowPrompt(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UpdateBlocksActivation(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_AutomaticBackgroundTask(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ForceUpdateFromAnyVersion(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsAutoRepairEnabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Version(
                    ABI::Windows::ApplicationModel::PackageVersion* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LastChecked(
                    ABI::Windows::Foundation::DateTime* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PausedUntil(
                    __FIReference_1_Windows__CFoundation__CDateTime** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UpdateUris(
                    __FIVectorView_1_Windows__CFoundation__CUri** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_RepairUris(
                    __FIVectorView_1_Windows__CFoundation__CUri** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DependencyPackageUris(
                    __FIVectorView_1_Windows__CFoundation__CUri** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_OptionalPackageUris(
                    __FIVectorView_1_Windows__CFoundation__CUri** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PolicySource(
                    ABI::Windows::ApplicationModel::AppInstallerPolicySource* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInstallerInfo2 = __uuidof(IAppInstallerInfo2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstance[] = L"Windows.ApplicationModel.IAppInstance";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("675f2b47-f25f-4532-9fd6-3633e0634d01")
            IAppInstance : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Key(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsCurrentInstance(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RedirectActivationTo(void) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInstance = __uuidof(IAppInstance);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstance;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstanceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstanceStatics[] = L"Windows.ApplicationModel.IAppInstanceStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("9d11e77f-9ea6-47af-a6ec-46784c5ba254")
            IAppInstanceStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_RecommendedInstance(
                    ABI::Windows::ApplicationModel::IAppInstance** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetActivatedEventArgs(
                    ABI::Windows::ApplicationModel::Activation::IActivatedEventArgs** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE FindOrRegisterInstanceForKey(
                    HSTRING key,
                    ABI::Windows::ApplicationModel::IAppInstance** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Unregister(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetInstances(
                    __FIVector_1_Windows__CApplicationModel__CAppInstance** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IAppInstanceStatics = __uuidof(IAppInstanceStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IDesignModeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DesignMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IDesignModeStatics[] = L"Windows.ApplicationModel.IDesignModeStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("2c3893cc-f81a-4e7a-b857-76a80887e185")
            IDesignModeStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_DesignModeEnabled(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IDesignModeStatics = __uuidof(IDesignModeStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIDesignModeStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IDesignModeStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DesignMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IDesignModeStatics2[] = L"Windows.ApplicationModel.IDesignModeStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("80cf8137-b064-4858-bec8-3eba22357535")
            IDesignModeStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_DesignMode2Enabled(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IDesignModeStatics2 = __uuidof(IDesignModeStatics2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.IEnteredBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IEnteredBackgroundEventArgs[] = L"Windows.ApplicationModel.IEnteredBackgroundEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("f722dcc2-9827-403d-aaed-ecca9ac17398")
            IEnteredBackgroundEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                    ABI::Windows::Foundation::IDeferral** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IEnteredBackgroundEventArgs = __uuidof(IEnteredBackgroundEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IFindRelatedPackagesOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FindRelatedPackagesOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFindRelatedPackagesOptions[] = L"Windows.ApplicationModel.IFindRelatedPackagesOptions";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("41dd7eea-b335-521f-b96c-5ea07f5b7329")
            IFindRelatedPackagesOptions : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Relationship(
                    ABI::Windows::ApplicationModel::PackageRelationship* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_Relationship(
                    ABI::Windows::ApplicationModel::PackageRelationship value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IncludeFrameworks(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IncludeFrameworks(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IncludeHostRuntimes(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IncludeHostRuntimes(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IncludeOptionals(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IncludeOptionals(
                    boolean value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IncludeResources(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE put_IncludeResources(
                    boolean value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IFindRelatedPackagesOptions = __uuidof(IFindRelatedPackagesOptions);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IFindRelatedPackagesOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FindRelatedPackagesOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFindRelatedPackagesOptionsFactory[] = L"Windows.ApplicationModel.IFindRelatedPackagesOptionsFactory";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("d7d17254-a4fd-55c4-98cf-f2710b7d8be2")
            IFindRelatedPackagesOptionsFactory : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE CreateInstance(
                    ABI::Windows::ApplicationModel::PackageRelationship Relationship,
                    ABI::Windows::ApplicationModel::IFindRelatedPackagesOptions** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IFindRelatedPackagesOptionsFactory = __uuidof(IFindRelatedPackagesOptionsFactory);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IFullTrustProcessLaunchResult
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FullTrustProcessLaunchResult
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFullTrustProcessLaunchResult[] = L"Windows.ApplicationModel.IFullTrustProcessLaunchResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("8917d888-edfb-515f-8e22-5ebceb69dfd9")
            IFullTrustProcessLaunchResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_LaunchResult(
                    ABI::Windows::ApplicationModel::FullTrustLaunchResult* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IFullTrustProcessLaunchResult = __uuidof(IFullTrustProcessLaunchResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.IFullTrustProcessLauncherStatics
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FullTrustProcessLauncher
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFullTrustProcessLauncherStatics[] = L"Windows.ApplicationModel.IFullTrustProcessLauncherStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("d784837f-1100-3c6b-a455-f6262cc331b6")
            IFullTrustProcessLauncherStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE LaunchFullTrustProcessForCurrentAppAsync(
                    ABI::Windows::Foundation::IAsyncAction** asyncAction
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE LaunchFullTrustProcessForCurrentAppWithParametersAsync(
                    HSTRING parameterGroupId,
                    ABI::Windows::Foundation::IAsyncAction** asyncAction
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE LaunchFullTrustProcessForAppAsync(
                    HSTRING fullTrustPackageRelativeAppId,
                    ABI::Windows::Foundation::IAsyncAction** asyncAction
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE LaunchFullTrustProcessForAppWithParametersAsync(
                    HSTRING fullTrustPackageRelativeAppId,
                    HSTRING parameterGroupId,
                    ABI::Windows::Foundation::IAsyncAction** asyncAction
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IFullTrustProcessLauncherStatics = __uuidof(IFullTrustProcessLauncherStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IFullTrustProcessLauncherStatics2
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FullTrustProcessLauncher
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFullTrustProcessLauncherStatics2[] = L"Windows.ApplicationModel.IFullTrustProcessLauncherStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("8b8ed72f-b65c-56cf-a1a7-2bf77cbc6ea8")
            IFullTrustProcessLauncherStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(
                    HSTRING commandLine,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE LaunchFullTrustProcessForAppWithArgumentsAsync(
                    HSTRING fullTrustPackageRelativeAppId,
                    HSTRING commandLine,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IFullTrustProcessLauncherStatics2 = __uuidof(IFullTrustProcessLauncherStatics2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.ILeavingBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ILeavingBackgroundEventArgs[] = L"Windows.ApplicationModel.ILeavingBackgroundEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("39c6ec9a-ae6e-46f9-a07a-cfc23f88733e")
            ILeavingBackgroundEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                    ABI::Windows::Foundation::IDeferral** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILeavingBackgroundEventArgs = __uuidof(ILeavingBackgroundEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.ILimitedAccessFeatureRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LimitedAccessFeatureRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ILimitedAccessFeatureRequestResult[] = L"Windows.ApplicationModel.ILimitedAccessFeatureRequestResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("d45156a6-1e24-5ddd-abb4-6188aba4d5bf")
            ILimitedAccessFeatureRequestResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_FeatureId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Status(
                    ABI::Windows::ApplicationModel::LimitedAccessFeatureStatus* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EstimatedRemovalDate(
                    __FIReference_1_Windows__CFoundation__CDateTime** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILimitedAccessFeatureRequestResult = __uuidof(ILimitedAccessFeatureRequestResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.ILimitedAccessFeaturesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LimitedAccessFeatures
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ILimitedAccessFeaturesStatics[] = L"Windows.ApplicationModel.ILimitedAccessFeaturesStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("8be612d4-302b-5fbf-a632-1a99e43e8925")
            ILimitedAccessFeaturesStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE TryUnlockFeature(
                    HSTRING featureId,
                    HSTRING token,
                    HSTRING attestation,
                    ABI::Windows::ApplicationModel::ILimitedAccessFeatureRequestResult** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ILimitedAccessFeaturesStatics = __uuidof(ILimitedAccessFeaturesStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage[] = L"Windows.ApplicationModel.IPackage";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("163c792f-bd75-413c-bf23-b1fe7b95d825")
            IPackage : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Id(
                    ABI::Windows::ApplicationModel::IPackageId** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_InstalledLocation(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsFramework(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Dependencies(
                    __FIVectorView_1_Windows__CApplicationModel__CPackage** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage = __uuidof(IPackage);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage2[] = L"Windows.ApplicationModel.IPackage2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("a6612fb6-7688-4ace-95fb-359538e7aa01")
            IPackage2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_DisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublisherDisplayName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Description(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Logo(
                    ABI::Windows::Foundation::IUriRuntimeClass** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsResourcePackage(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsBundle(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsDevelopmentMode(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage2 = __uuidof(IPackage2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage3[] = L"Windows.ApplicationModel.IPackage3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("5f738b61-f86a-4917-93d1-f1ee9d3b35d9")
            IPackage3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Status(
                    ABI::Windows::ApplicationModel::IPackageStatus** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_InstalledDate(
                    ABI::Windows::Foundation::DateTime* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetAppListEntriesAsync(
                    __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage3 = __uuidof(IPackage3);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackage4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage4[] = L"Windows.ApplicationModel.IPackage4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("65aed1ae-b95b-450c-882b-6255187f397e")
            IPackage4 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_SignatureKind(
                    ABI::Windows::ApplicationModel::PackageSignatureKind* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsOptional(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE VerifyContentIntegrityAsync(
                    __FIAsyncOperation_1_boolean** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage4 = __uuidof(IPackage4);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackage5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage5[] = L"Windows.ApplicationModel.IPackage5";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("0e842dd4-d9ac-45ed-9a1e-74ce056b2635")
            IPackage5 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetContentGroupsAsync(
                    __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetContentGroupAsync(
                    HSTRING name,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE StageContentGroupsAsync(
                    __FIIterable_1_HSTRING* names,
                    __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE StageContentGroupsWithPriorityAsync(
                    __FIIterable_1_HSTRING* names,
                    boolean moveToHeadOfQueue,
                    __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE SetInUseAsync(
                    boolean inUse,
                    __FIAsyncOperation_1_boolean** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage5 = __uuidof(IPackage5);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackage6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage6[] = L"Windows.ApplicationModel.IPackage6";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("8b1ad942-12d7-4754-ae4e-638cbc0e3a2e")
            IPackage6 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetAppInstallerInfo(
                    ABI::Windows::ApplicationModel::IAppInstallerInfo** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE CheckUpdateAvailabilityAsync(
                    __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage6 = __uuidof(IPackage6);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage6;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IPackage7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage7[] = L"Windows.ApplicationModel.IPackage7";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("86ff8d31-a2e4-45e0-9732-283a6d88fde1")
            IPackage7 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_MutableLocation(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EffectiveLocation(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage7 = __uuidof(IPackage7);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage7;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.IPackage8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage8[] = L"Windows.ApplicationModel.IPackage8";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("2c584f7b-ce2a-4be6-a093-77cfbb2a7ea1")
            IPackage8 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_EffectiveExternalLocation(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MachineExternalLocation(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UserExternalLocation(
                    ABI::Windows::Storage::IStorageFolder** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_InstalledPath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MutablePath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EffectivePath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_EffectiveExternalPath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_MachineExternalPath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_UserExternalPath(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetLogoAsRandomAccessStreamReference(
                    ABI::Windows::Foundation::Size size,
                    ABI::Windows::Storage::Streams::IRandomAccessStreamReference** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetAppListEntries(
                    __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsStub(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage8 = __uuidof(IPackage8);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage8;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.IPackage9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage9[] = L"Windows.ApplicationModel.IPackage9";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("d5ab224f-d7e1-49ec-90ce-720cdbd02e9c")
            IPackage9 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE FindRelatedPackages(
                    ABI::Windows::ApplicationModel::IFindRelatedPackagesOptions* options,
                    __FIVector_1_Windows__CApplicationModel__CPackage** result
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SourceUriSchemeName(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackage9 = __uuidof(IPackage9);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage9;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog[] = L"Windows.ApplicationModel.IPackageCatalog";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("230a3751-9de3-4445-be74-91fb325abefe")
            IPackageCatalog : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE add_PackageStaging(
                    __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PackageStaging(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PackageInstalling(
                    __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PackageInstalling(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PackageUpdating(
                    __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PackageUpdating(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PackageUninstalling(
                    __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PackageUninstalling(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE add_PackageStatusChanged(
                    __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PackageStatusChanged(
                    EventRegistrationToken token
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalog = __uuidof(IPackageCatalog);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog2[] = L"Windows.ApplicationModel.IPackageCatalog2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("96a60c36-8ff7-4344-b6bf-ee64c2207ed2")
            IPackageCatalog2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE add_PackageContentGroupStaging(
                    __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs* handler,
                    EventRegistrationToken* token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE remove_PackageContentGroupStaging(
                    EventRegistrationToken token
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE AddOptionalPackageAsync(
                    HSTRING optionalPackageFamilyName,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalog2 = __uuidof(IPackageCatalog2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog3[] = L"Windows.ApplicationModel.IPackageCatalog3";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("96dd5c88-8837-43f9-9015-033434ba14f3")
            IPackageCatalog3 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RemoveOptionalPackagesAsync(
                    __FIIterable_1_HSTRING* optionalPackageFamilyNames,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalog3 = __uuidof(IPackageCatalog3);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog4[] = L"Windows.ApplicationModel.IPackageCatalog4";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("c37c399b-44cc-4b7b-8baf-796c04ead3b9")
            IPackageCatalog4 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE AddResourcePackageAsync(
                    HSTRING resourcePackageFamilyName,
                    HSTRING resourceID,
                    ABI::Windows::ApplicationModel::AddResourcePackageOptions options,
                    __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE RemoveResourcePackagesAsync(
                    __FIIterable_1_Windows__CApplicationModel__CPackage* resourcePackages,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalog4 = __uuidof(IPackageCatalog4);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogAddOptionalPackageResult[] = L"Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("3bf10cd4-b4df-47b3-a963-e2fa832f7dd3")
            IPackageCatalogAddOptionalPackageResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalogAddOptionalPackageResult = __uuidof(IPackageCatalogAddOptionalPackageResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogAddResourcePackageResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogAddResourcePackageResult[] = L"Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("9636ce0d-3e17-493f-aa08-ccec6fdef699")
            IPackageCatalogAddResourcePackageResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalogAddResourcePackageResult = __uuidof(IPackageCatalogAddResourcePackageResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogRemoveOptionalPackagesResult[] = L"Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("29d2f97b-d974-4e64-9359-22cadfd79828")
            IPackageCatalogRemoveOptionalPackagesResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_PackagesRemoved(
                    __FIVectorView_1_Windows__CApplicationModel__CPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalogRemoveOptionalPackagesResult = __uuidof(IPackageCatalogRemoveOptionalPackagesResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogRemoveResourcePackagesResult[] = L"Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("ae719709-1a52-4321-87b3-e5a1a17981a7")
            IPackageCatalogRemoveResourcePackagesResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_PackagesRemoved(
                    __FIVectorView_1_Windows__CApplicationModel__CPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalogRemoveResourcePackagesResult = __uuidof(IPackageCatalogRemoveResourcePackagesResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogStatics[] = L"Windows.ApplicationModel.IPackageCatalogStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("a18c9696-e65b-4634-ba21-5e63eb7244a7")
            IPackageCatalogStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE OpenForCurrentPackage(
                    ABI::Windows::ApplicationModel::IPackageCatalog** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE OpenForCurrentUser(
                    ABI::Windows::ApplicationModel::IPackageCatalog** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalogStatics = __uuidof(IPackageCatalogStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogStatics2[] = L"Windows.ApplicationModel.IPackageCatalogStatics2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("4c11c159-9a28-598c-b185-55e1899b2be4")
            IPackageCatalogStatics2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE OpenForPackage(
                    ABI::Windows::ApplicationModel::IPackage* package,
                    ABI::Windows::ApplicationModel::IPackageCatalog** result
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageCatalogStatics2 = __uuidof(IPackageCatalogStatics2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IPackageContentGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageContentGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageContentGroup[] = L"Windows.ApplicationModel.IPackageContentGroup";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("8f62695d-120a-4798-b5e1-5800dda8f2e1")
            IPackageContentGroup : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Name(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_State(
                    ABI::Windows::ApplicationModel::PackageContentGroupState* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsRequired(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageContentGroup = __uuidof(IPackageContentGroup);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageContentGroup;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageContentGroupStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageContentGroupStagingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageContentGroupStagingEventArgs[] = L"Windows.ApplicationModel.IPackageContentGroupStagingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("3d7bc27e-6f27-446c-986e-d4733d4d9113")
            IPackageContentGroupStagingEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Progress(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                    HRESULT* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ContentGroupName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsContentGroupRequired(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageContentGroupStagingEventArgs = __uuidof(IPackageContentGroupStagingEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageContentGroupStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageContentGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageContentGroupStatics[] = L"Windows.ApplicationModel.IPackageContentGroupStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("70ee7619-5f12-4b92-b9ea-6ccada13bc75")
            IPackageContentGroupStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_RequiredGroupName(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageContentGroupStatics = __uuidof(IPackageContentGroupStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageId[] = L"Windows.ApplicationModel.IPackageId";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("1adb665e-37c7-4790-9980-dd7ae74e8bb2")
            IPackageId : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Name(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Version(
                    ABI::Windows::ApplicationModel::PackageVersion* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Architecture(
                    ABI::Windows::System::ProcessorArchitecture* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ResourceId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Publisher(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PublisherId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FullName(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_FamilyName(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageId = __uuidof(IPackageId);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageId;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageIdWithMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageIdWithMetadata[] = L"Windows.ApplicationModel.IPackageIdWithMetadata";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("40577a7c-0c9e-443d-9074-855f5ce0a08d")
            IPackageIdWithMetadata : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ProductId(
                    HSTRING* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Author(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageIdWithMetadata = __uuidof(IPackageIdWithMetadata);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageInstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageInstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageInstallingEventArgs[] = L"Windows.ApplicationModel.IPackageInstallingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("97741eb7-ab7a-401a-8b61-eb0e7faff237")
            IPackageInstallingEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Progress(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageInstallingEventArgs = __uuidof(IPackageInstallingEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStagingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStagingEventArgs[] = L"Windows.ApplicationModel.IPackageStagingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("1041682d-54e2-4f51-b828-9ef7046c210f")
            IPackageStagingEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Progress(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageStagingEventArgs = __uuidof(IPackageStagingEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatics[] = L"Windows.ApplicationModel.IPackageStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("4e534bdf-2960-4878-97a4-9624deb72f2d")
            IPackageStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Current(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageStatics = __uuidof(IPackageStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatus[] = L"Windows.ApplicationModel.IPackageStatus";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("5fe74f71-a365-4c09-a02d-046d525ea1da")
            IPackageStatus : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE VerifyIsOK(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NotAvailable(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_PackageOffline(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DataOffline(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Disabled(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_NeedsRemediation(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_LicenseIssue(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Modified(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Tampered(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DependencyIssue(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Servicing(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_DeploymentInProgress(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageStatus = __uuidof(IPackageStatus);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatus;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatus2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatus2[] = L"Windows.ApplicationModel.IPackageStatus2";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("f428fa93-7c56-4862-acfa-abaedcc0694d")
            IPackageStatus2 : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_IsPartiallyStaged(
                    boolean* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageStatus2 = __uuidof(IPackageStatus2);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatus2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.IPackageStatusChangedEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("437d714d-bd80-4a70-bc50-f6e796509575")
            IPackageStatusChangedEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageStatusChangedEventArgs = __uuidof(IPackageStatusChangedEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageUninstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.IPackageUninstallingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("4443aa52-ab22-44cd-82bb-4ec9b827367a")
            IPackageUninstallingEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Package(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Progress(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageUninstallingEventArgs = __uuidof(IPackageUninstallingEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageUpdateAvailabilityResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageUpdateAvailabilityResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageUpdateAvailabilityResult[] = L"Windows.ApplicationModel.IPackageUpdateAvailabilityResult";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("114e5009-199a-48a1-a079-313c45634a71")
            IPackageUpdateAvailabilityResult : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_Availability(
                    ABI::Windows::ApplicationModel::PackageUpdateAvailability* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ExtendedError(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageUpdateAvailabilityResult = __uuidof(IPackageUpdateAvailabilityResult);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.IPackageUpdatingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("cd7b4228-fd74-443e-b114-23e677b0e86f")
            IPackageUpdatingEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_ActivityId(
                    GUID* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_SourcePackage(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TargetPackage(
                    ABI::Windows::ApplicationModel::IPackage** value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Progress(
                    DOUBLE* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_IsComplete(
                    boolean* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_ErrorCode(
                    HRESULT* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageUpdatingEventArgs = __uuidof(IPackageUpdatingEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageWithMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageWithMetadata[] = L"Windows.ApplicationModel.IPackageWithMetadata";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("95949780-1de9-40f2-b452-0de9f1910012")
            IPackageWithMetadata : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_InstallDate(
                    ABI::Windows::Foundation::DateTime* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetThumbnailToken(
                    HSTRING* value
                    ) = 0;
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                DEPRECATED("Launch may be altered or unavailable for releases after Windows 8.1. Instead, for SmartCardTrigger scenarios use SmartCardTriggerDetails.TryLaunchSelfAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
                virtual HRESULT STDMETHODCALLTYPE Launch(
                    HSTRING parameters
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IPackageWithMetadata = __uuidof(IPackageWithMetadata);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IStartupTask
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.StartupTask
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IStartupTask[] = L"Windows.ApplicationModel.IStartupTask";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("f75c23c8-b5f2-4f6c-88dd-36cb1d599d17")
            IStartupTask : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE RequestEnableAsync(
                    __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE Disable(void) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_State(
                    ABI::Windows::ApplicationModel::StartupTaskState* value
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_TaskId(
                    HSTRING* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStartupTask = __uuidof(IStartupTask);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIStartupTask;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IStartupTaskStatics
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.StartupTask
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IStartupTaskStatics[] = L"Windows.ApplicationModel.IStartupTaskStatics";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("ee5b60bd-a148-41a7-b26e-e8b88a1e62f8")
            IStartupTaskStatics : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetForCurrentPackageAsync(
                    __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask** operation
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE GetAsync(
                    HSTRING taskId,
                    __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask** operation
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_IStartupTaskStatics = __uuidof(IStartupTaskStatics);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ISuspendingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ISuspendingDeferral[] = L"Windows.ApplicationModel.ISuspendingDeferral";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("59140509-8bc9-4eb4-b636-dabdc4f46f66")
            ISuspendingDeferral : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE Complete(void) = 0;
            };

            MIDL_CONST_ID IID& IID_ISuspendingDeferral = __uuidof(ISuspendingDeferral);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CISuspendingDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ISuspendingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ISuspendingEventArgs[] = L"Windows.ApplicationModel.ISuspendingEventArgs";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("96061c05-2dba-4d08-b0bd-2b30a131c6aa")
            ISuspendingEventArgs : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE get_SuspendingOperation(
                    ABI::Windows::ApplicationModel::ISuspendingOperation** value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISuspendingEventArgs = __uuidof(ISuspendingEventArgs);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ISuspendingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ISuspendingOperation[] = L"Windows.ApplicationModel.ISuspendingOperation";
namespace ABI {
    namespace Windows {
        namespace ApplicationModel {
            MIDL_INTERFACE("9da4ca41-20e1-4e9b-9f65-a9f435340c3a")
            ISuspendingOperation : public IInspectable
            {
            public:
                virtual HRESULT STDMETHODCALLTYPE GetDeferral(
                    ABI::Windows::ApplicationModel::ISuspendingDeferral** deferral
                    ) = 0;
                virtual HRESULT STDMETHODCALLTYPE get_Deadline(
                    ABI::Windows::Foundation::DateTime* value
                    ) = 0;
            };

            MIDL_CONST_ID IID& IID_ISuspendingOperation = __uuidof(ISuspendingOperation);
        } /* ApplicationModel */
    } /* Windows */
} /* ABI */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CISuspendingOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppDisplayInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppDisplayInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppDisplayInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppDisplayInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppDisplayInfo[] = L"Windows.ApplicationModel.AppDisplayInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IAppInfoStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppInfo ** Default Interface **
 *    Windows.ApplicationModel.IAppInfo2
 *    Windows.ApplicationModel.IAppInfo3
 *    Windows.ApplicationModel.IAppInfo4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppInfo[] = L"Windows.ApplicationModel.AppInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppInstallerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppInstallerInfo ** Default Interface **
 *    Windows.ApplicationModel.IAppInstallerInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppInstallerInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppInstallerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppInstallerInfo[] = L"Windows.ApplicationModel.AppInstallerInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.AppInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IAppInstanceStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppInstance ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppInstance_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppInstance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppInstance[] = L"Windows.ApplicationModel.AppInstance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.DesignMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IDesignModeStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.IDesignModeStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DesignMode_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DesignMode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DesignMode[] = L"Windows.ApplicationModel.DesignMode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.EnteredBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IEnteredBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_EnteredBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_EnteredBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_EnteredBackgroundEventArgs[] = L"Windows.ApplicationModel.EnteredBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.FindRelatedPackagesOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.IFindRelatedPackagesOptionsFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IFindRelatedPackagesOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_FindRelatedPackagesOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_FindRelatedPackagesOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_FindRelatedPackagesOptions[] = L"Windows.ApplicationModel.FindRelatedPackagesOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.ApplicationModel.FullTrustProcessLaunchResult
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IFullTrustProcessLaunchResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLaunchResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLaunchResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_FullTrustProcessLaunchResult[] = L"Windows.ApplicationModel.FullTrustProcessLaunchResult";
#endif
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.FullTrustProcessLauncher
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IFullTrustProcessLauncherStatics2 interface starting with version 2.0 of the Windows.ApplicationModel.FullTrustAppContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.IFullTrustProcessLauncherStatics interface starting with version 1.0 of the Windows.ApplicationModel.FullTrustAppContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLauncher_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLauncher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_FullTrustProcessLauncher[] = L"Windows.ApplicationModel.FullTrustProcessLauncher";
#endif
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LeavingBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ILeavingBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LeavingBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LeavingBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LeavingBackgroundEventArgs[] = L"Windows.ApplicationModel.LeavingBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.LimitedAccessFeatureRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ILimitedAccessFeatureRequestResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatureRequestResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatureRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LimitedAccessFeatureRequestResult[] = L"Windows.ApplicationModel.LimitedAccessFeatureRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.LimitedAccessFeatures
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.ILimitedAccessFeaturesStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatures_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatures_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LimitedAccessFeatures[] = L"Windows.ApplicationModel.LimitedAccessFeatures";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.Package
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IPackageStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackage ** Default Interface **
 *    Windows.ApplicationModel.IPackage2
 *    Windows.ApplicationModel.IPackage3
 *    Windows.ApplicationModel.IPackageWithMetadata
 *    Windows.ApplicationModel.IPackage4
 *    Windows.ApplicationModel.IPackage5
 *    Windows.ApplicationModel.IPackage6
 *    Windows.ApplicationModel.IPackage7
 *    Windows.ApplicationModel.IPackage8
 *    Windows.ApplicationModel.IPackage9
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Package_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Package_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Package[] = L"Windows.ApplicationModel.Package";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IPackageCatalogStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.IPackageCatalogStatics2 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalog ** Default Interface **
 *    Windows.ApplicationModel.IPackageCatalog2
 *    Windows.ApplicationModel.IPackageCatalog3
 *    Windows.ApplicationModel.IPackageCatalog4
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalog[] = L"Windows.ApplicationModel.PackageCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddOptionalPackageResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddOptionalPackageResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogAddOptionalPackageResult[] = L"Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogAddResourcePackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddResourcePackageResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddResourcePackageResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogAddResourcePackageResult[] = L"Windows.ApplicationModel.PackageCatalogAddResourcePackageResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveOptionalPackagesResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveOptionalPackagesResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogRemoveOptionalPackagesResult[] = L"Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveResourcePackagesResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveResourcePackagesResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogRemoveResourcePackagesResult[] = L"Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.PackageContentGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IPackageContentGroupStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageContentGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroup_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageContentGroup[] = L"Windows.ApplicationModel.PackageContentGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.PackageContentGroupStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageContentGroupStagingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroupStagingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroupStagingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageContentGroupStagingEventArgs[] = L"Windows.ApplicationModel.PackageContentGroupStagingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.PackageId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageId ** Default Interface **
 *    Windows.ApplicationModel.IPackageIdWithMetadata
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageId_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageId[] = L"Windows.ApplicationModel.PackageId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.PackageInstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageInstallingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageInstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageInstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageInstallingEventArgs[] = L"Windows.ApplicationModel.PackageInstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageStagingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageStagingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageStagingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageStagingEventArgs[] = L"Windows.ApplicationModel.PackageStagingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageStatus ** Default Interface **
 *    Windows.ApplicationModel.IPackageStatus2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageStatus_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageStatus[] = L"Windows.ApplicationModel.PackageStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.PackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageStatusChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.PackageStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageUninstallingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageUninstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageUninstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageUninstallingEventArgs[] = L"Windows.ApplicationModel.PackageUninstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageUpdateAvailabilityResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageUpdateAvailabilityResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageUpdateAvailabilityResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageUpdateAvailabilityResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageUpdateAvailabilityResult[] = L"Windows.ApplicationModel.PackageUpdateAvailabilityResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.PackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageUpdatingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageUpdatingEventArgs[] = L"Windows.ApplicationModel.PackageUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.StartupTask
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IStartupTaskStatics interface starting with version 1.0 of the Windows.ApplicationModel.StartupTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IStartupTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_StartupTask_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_StartupTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_StartupTask[] = L"Windows.ApplicationModel.StartupTask";
#endif
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SuspendingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SuspendingDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SuspendingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SuspendingDeferral[] = L"Windows.ApplicationModel.SuspendingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SuspendingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SuspendingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SuspendingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SuspendingEventArgs[] = L"Windows.ApplicationModel.SuspendingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SuspendingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SuspendingOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SuspendingOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SuspendingOperation[] = L"Windows.ApplicationModel.SuspendingOperation";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#else // !defined(__cplusplus)
/* Forward Declarations */
#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo __x_ABI_CWindows_CApplicationModel_CIAppInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo2 __x_ABI_CWindows_CApplicationModel_CIAppInfo2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo3 __x_ABI_CWindows_CApplicationModel_CIAppInfo3;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfo4 __x_ABI_CWindows_CApplicationModel_CIAppInfo4;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2 __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstance_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstance_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInstance __x_ABI_CWindows_CApplicationModel_CIAppInstance;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstance_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2 __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions;

#endif // ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory;

#endif // ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2 __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage __x_ABI_CWindows_CApplicationModel_CIPackage;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage2 __x_ABI_CWindows_CApplicationModel_CIPackage2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage3 __x_ABI_CWindows_CApplicationModel_CIPackage3;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage4 __x_ABI_CWindows_CApplicationModel_CIPackage4;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage5_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage5_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage5 __x_ABI_CWindows_CApplicationModel_CIPackage5;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage5_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage6_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage6_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage6 __x_ABI_CWindows_CApplicationModel_CIPackage6;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage6_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage7_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage7_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage7 __x_ABI_CWindows_CApplicationModel_CIPackage7;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage7_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage8_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage8_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage8 __x_ABI_CWindows_CApplicationModel_CIPackage8;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage8_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackage9_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackage9_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackage9 __x_ABI_CWindows_CApplicationModel_CIPackage9;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackage9_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog __x_ABI_CWindows_CApplicationModel_CIPackageCatalog;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2 __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3 __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4 __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2 __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageId_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageId_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageId __x_ABI_CWindows_CApplicationModel_CIPackageId;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageId_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageStatics __x_ABI_CWindows_CApplicationModel_CIPackageStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageStatus __x_ABI_CWindows_CApplicationModel_CIPackageStatus;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageStatus2 __x_ABI_CWindows_CApplicationModel_CIPackageStatus2;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata;

#endif // ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIStartupTask_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTask_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIStartupTask __x_ABI_CWindows_CApplicationModel_CIStartupTask;

#endif // ____x_ABI_CWindows_CApplicationModel_CIStartupTask_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics;

#endif // ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CISuspendingOperation __x_ABI_CWindows_CApplicationModel_CISuspendingOperation;

#endif // ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_FWD_DEFINED__

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

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult;

#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResultVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResultVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask;

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* This,
        __x_ABI_CWindows_CApplicationModel_CIStartupTask** result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

typedef enum __x_ABI_CWindows_CApplicationModel_CStartupTaskState __x_ABI_CWindows_CApplicationModel_CStartupTaskState;

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState;

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState;

typedef struct __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* This,
        enum __x_ABI_CWindows_CApplicationModel_CStartupTaskState* result);

    END_INTERFACE
} __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskStateVtbl;

interface __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState
{
    CONST_VTBL struct __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState;

typedef struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskStateVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskStateVtbl;

interface __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskStateVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1_Windows__CApplicationModel__CStartupTaskState_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#ifndef ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry;

#endif // ____x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry_FWD_DEFINED__

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry;

typedef struct __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry;

typedef struct __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __FIIterator_1_Windows__CApplicationModel__CCore__CAppListEntry** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CCore_CIAppListEntry** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CStartupTask __FIIterator_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIIterator_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        __x_ABI_CWindows_CApplicationModel_CIStartupTask** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CStartupTask* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIStartupTask** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CStartupTask_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CStartupTask __FIIterable_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIIterable_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CStartupTask* This,
        __FIIterator_1_Windows__CApplicationModel__CStartupTask** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CStartupTask_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CStartupTask __FIVectorView_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIStartupTask** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        __x_ABI_CWindows_CApplicationModel_CIStartupTask* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIStartupTask** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask;

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        __FIVectorView_1_Windows__CApplicationModel__CStartupTask** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask_INTERFACE_DEFINED__
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIIterator_1_Windows__CApplicationModel__CPackageContentGroup** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CPackageContentGroup __FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackageContentGroup** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIVector_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CPackageContentGroup_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIVector_1_Windows__CApplicationModel__CPackageContentGroup** result);

    END_INTERFACE
} __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__)
#define ____FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup;

typedef struct __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* This,
        __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl;

interface __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup
{
    CONST_VTBL struct __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationCompletedHandler_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

typedef struct __x_ABI_CWindows_CApplicationModel_CPackageInstallProgress __x_ABI_CWindows_CApplicationModel_CPackageInstallProgress;

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

typedef interface __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

typedef struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* asyncInfo,
        AsyncStatus asyncStatus);

    END_INTERFACE
} __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl;

interface __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_Invoke(This, asyncInfo, asyncStatus) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, asyncStatus))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

typedef struct __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* put_Progress)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress** result);
    HRESULT (STDMETHODCALLTYPE* put_Completed)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* handler);
    HRESULT (STDMETHODCALLTYPE* get_Completed)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __FIAsyncOperationWithProgressCompletedHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress** result);
    HRESULT (STDMETHODCALLTYPE* GetResults)(__FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult** result);

    END_INTERFACE
} __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl;

interface __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress
{
    CONST_VTBL struct __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_put_Progress(This, handler) \
    ((This)->lpVtbl->put_Progress(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_get_Progress(This, result) \
    ((This)->lpVtbl->get_Progress(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_put_Completed(This, handler) \
    ((This)->lpVtbl->put_Completed(This, handler))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_get_Completed(This, result) \
    ((This)->lpVtbl->get_Completed(This, result))

#define __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_GetResults(This, result) \
    ((This)->lpVtbl->GetResults(This, result))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__)
#define ____FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__

typedef interface __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress;

typedef struct __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* This,
        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress* asyncInfo,
        struct __x_ABI_CWindows_CApplicationModel_CPackageInstallProgress progressInfo);

    END_INTERFACE
} __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl;

interface __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress
{
    CONST_VTBL struct __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgressVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_Invoke(This, asyncInfo, progressInfo) \
    ((This)->lpVtbl->Invoke(This, asyncInfo, progressInfo))

#endif /* COBJMACROS */

#endif // ____FIAsyncOperationProgressHandler_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CAppInstance __FIIterator_1_Windows__CApplicationModel__CAppInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CAppInstance;

typedef struct __FIIterator_1_Windows__CApplicationModel__CAppInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CAppInstanceVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CAppInstance
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CAppInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CAppInstance_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CAppInstance __FIIterable_1_Windows__CApplicationModel__CAppInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CAppInstance;

typedef struct __FIIterable_1_Windows__CApplicationModel__CAppInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CAppInstance* This,
        __FIIterator_1_Windows__CApplicationModel__CAppInstance** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CAppInstanceVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CAppInstance
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CAppInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CAppInstance_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterator_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__)
#define ____FIIterator_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__

typedef interface __FIIterator_1_Windows__CApplicationModel__CPackage __FIIterator_1_Windows__CApplicationModel__CPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterator_1_Windows__CApplicationModel__CPackage;

typedef struct __FIIterator_1_Windows__CApplicationModel__CPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterator_1_Windows__CApplicationModel__CPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterator_1_Windows__CApplicationModel__CPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** result);
    HRESULT (STDMETHODCALLTYPE* get_HasCurrent)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* MoveNext)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIIterator_1_Windows__CApplicationModel__CPackage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackage** items,
        UINT32* result);

    END_INTERFACE
} __FIIterator_1_Windows__CApplicationModel__CPackageVtbl;

interface __FIIterator_1_Windows__CApplicationModel__CPackage
{
    CONST_VTBL struct __FIIterator_1_Windows__CApplicationModel__CPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterator_1_Windows__CApplicationModel__CPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_get_Current(This, result) \
    ((This)->lpVtbl->get_Current(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_get_HasCurrent(This, result) \
    ((This)->lpVtbl->get_HasCurrent(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_MoveNext(This, result) \
    ((This)->lpVtbl->MoveNext(This, result))

#define __FIIterator_1_Windows__CApplicationModel__CPackage_GetMany(This, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIIterator_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIIterable_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__)
#define ____FIIterable_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__

typedef interface __FIIterable_1_Windows__CApplicationModel__CPackage __FIIterable_1_Windows__CApplicationModel__CPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIIterable_1_Windows__CApplicationModel__CPackage;

typedef struct __FIIterable_1_Windows__CApplicationModel__CPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIIterable_1_Windows__CApplicationModel__CPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIIterable_1_Windows__CApplicationModel__CPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIIterable_1_Windows__CApplicationModel__CPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIIterable_1_Windows__CApplicationModel__CPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIIterable_1_Windows__CApplicationModel__CPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIIterable_1_Windows__CApplicationModel__CPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* First)(__FIIterable_1_Windows__CApplicationModel__CPackage* This,
        __FIIterator_1_Windows__CApplicationModel__CPackage** result);

    END_INTERFACE
} __FIIterable_1_Windows__CApplicationModel__CPackageVtbl;

interface __FIIterable_1_Windows__CApplicationModel__CPackage
{
    CONST_VTBL struct __FIIterable_1_Windows__CApplicationModel__CPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIIterable_1_Windows__CApplicationModel__CPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIIterable_1_Windows__CApplicationModel__CPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIIterable_1_Windows__CApplicationModel__CPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIIterable_1_Windows__CApplicationModel__CPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIIterable_1_Windows__CApplicationModel__CPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIIterable_1_Windows__CApplicationModel__CPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIIterable_1_Windows__CApplicationModel__CPackage_First(This, result) \
    ((This)->lpVtbl->First(This, result))

#endif /* COBJMACROS */

#endif // ____FIIterable_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CAppInstance __FIVectorView_1_Windows__CApplicationModel__CAppInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CAppInstance;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CAppInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CAppInstanceVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CAppInstance
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CAppInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CAppInstance_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVectorView_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__)
#define ____FIVectorView_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__

typedef interface __FIVectorView_1_Windows__CApplicationModel__CPackage __FIVectorView_1_Windows__CApplicationModel__CPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVectorView_1_Windows__CApplicationModel__CPackage;

typedef struct __FIVectorView_1_Windows__CApplicationModel__CPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVectorView_1_Windows__CApplicationModel__CPackage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackage** items,
        UINT32* result);

    END_INTERFACE
} __FIVectorView_1_Windows__CApplicationModel__CPackageVtbl;

interface __FIVectorView_1_Windows__CApplicationModel__CPackage
{
    CONST_VTBL struct __FIVectorView_1_Windows__CApplicationModel__CPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVectorView_1_Windows__CApplicationModel__CPackage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#endif /* COBJMACROS */

#endif // ____FIVectorView_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____FIVector_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CAppInstance __FIVector_1_Windows__CApplicationModel__CAppInstance;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CAppInstance;

typedef struct __FIVector_1_Windows__CApplicationModel__CAppInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        __FIVectorView_1_Windows__CApplicationModel__CAppInstance** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CAppInstance* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CAppInstanceVtbl;

interface __FIVector_1_Windows__CApplicationModel__CAppInstance
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CAppInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CAppInstance_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CAppInstance_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____FIVector_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__)
#define ____FIVector_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__

typedef interface __FIVector_1_Windows__CApplicationModel__CPackage __FIVector_1_Windows__CApplicationModel__CPackage;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FIVector_1_Windows__CApplicationModel__CPackage;

typedef struct __FIVector_1_Windows__CApplicationModel__CPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FIVector_1_Windows__CApplicationModel__CPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FIVector_1_Windows__CApplicationModel__CPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAt)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackage** result);
    HRESULT (STDMETHODCALLTYPE* get_Size)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* GetView)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackage** result);
    HRESULT (STDMETHODCALLTYPE* IndexOf)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage* value,
        UINT32* index,
        boolean* result);
    HRESULT (STDMETHODCALLTYPE* SetAt)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackage* value);
    HRESULT (STDMETHODCALLTYPE* InsertAt)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32 index,
        __x_ABI_CWindows_CApplicationModel_CIPackage* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAt)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32 index);
    HRESULT (STDMETHODCALLTYPE* Append)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage* value);
    HRESULT (STDMETHODCALLTYPE* RemoveAtEnd)(__FIVector_1_Windows__CApplicationModel__CPackage* This);
    HRESULT (STDMETHODCALLTYPE* Clear)(__FIVector_1_Windows__CApplicationModel__CPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetMany)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32 startIndex,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackage** items,
        UINT32* result);
    HRESULT (STDMETHODCALLTYPE* ReplaceAll)(__FIVector_1_Windows__CApplicationModel__CPackage* This,
        UINT32 itemsLength,
        __x_ABI_CWindows_CApplicationModel_CIPackage** items);

    END_INTERFACE
} __FIVector_1_Windows__CApplicationModel__CPackageVtbl;

interface __FIVector_1_Windows__CApplicationModel__CPackage
{
    CONST_VTBL struct __FIVector_1_Windows__CApplicationModel__CPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FIVector_1_Windows__CApplicationModel__CPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FIVector_1_Windows__CApplicationModel__CPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FIVector_1_Windows__CApplicationModel__CPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FIVector_1_Windows__CApplicationModel__CPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __FIVector_1_Windows__CApplicationModel__CPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __FIVector_1_Windows__CApplicationModel__CPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __FIVector_1_Windows__CApplicationModel__CPackage_GetAt(This, index, result) \
    ((This)->lpVtbl->GetAt(This, index, result))

#define __FIVector_1_Windows__CApplicationModel__CPackage_get_Size(This, result) \
    ((This)->lpVtbl->get_Size(This, result))

#define __FIVector_1_Windows__CApplicationModel__CPackage_GetView(This, result) \
    ((This)->lpVtbl->GetView(This, result))

#define __FIVector_1_Windows__CApplicationModel__CPackage_IndexOf(This, value, index, result) \
    ((This)->lpVtbl->IndexOf(This, value, index, result))

#define __FIVector_1_Windows__CApplicationModel__CPackage_SetAt(This, index, value) \
    ((This)->lpVtbl->SetAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CPackage_InsertAt(This, index, value) \
    ((This)->lpVtbl->InsertAt(This, index, value))

#define __FIVector_1_Windows__CApplicationModel__CPackage_RemoveAt(This, index) \
    ((This)->lpVtbl->RemoveAt(This, index))

#define __FIVector_1_Windows__CApplicationModel__CPackage_Append(This, value) \
    ((This)->lpVtbl->Append(This, value))

#define __FIVector_1_Windows__CApplicationModel__CPackage_RemoveAtEnd(This) \
    ((This)->lpVtbl->RemoveAtEnd(This))

#define __FIVector_1_Windows__CApplicationModel__CPackage_Clear(This) \
    ((This)->lpVtbl->Clear(This))

#define __FIVector_1_Windows__CApplicationModel__CPackage_GetMany(This, startIndex, itemsLength, items, result) \
    ((This)->lpVtbl->GetMany(This, startIndex, itemsLength, items, result))

#define __FIVector_1_Windows__CApplicationModel__CPackage_ReplaceAll(This, itemsLength, items) \
    ((This)->lpVtbl->ReplaceAll(This, itemsLength, items))

#endif /* COBJMACROS */

#endif // ____FIVector_1_Windows__CApplicationModel__CPackage_INTERFACE_DEFINED__
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

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_INTERFACE_DEFINED__

typedef interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs;

//  Declare the parameterized interface IID.
EXTERN_C const IID IID___FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs;

typedef struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* Invoke)(__FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog* sender,
        __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* args);

    END_INTERFACE
} __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgsVtbl;

interface __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs
{
    CONST_VTBL struct __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_Invoke(This, sender, args) \
    ((This)->lpVtbl->Invoke(This, sender, args))

#endif /* COBJMACROS */

#endif // ____FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs_INTERFACE_DEFINED__
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

#ifndef ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
#define ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs;

#endif // ____x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIDeferral __x_ABI_CWindows_CFoundation_CIDeferral;

#endif // ____x_ABI_CWindows_CFoundation_CIDeferral_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIAsyncAction __x_ABI_CWindows_CFoundation_CIAsyncAction;

#endif // ____x_ABI_CWindows_CFoundation_CIAsyncAction_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
#define ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CFoundation_CIPropertyValue __x_ABI_CWindows_CFoundation_CIPropertyValue;

#endif // ____x_ABI_CWindows_CFoundation_CIPropertyValue_FWD_DEFINED__

typedef struct __x_ABI_CWindows_CFoundation_CSize __x_ABI_CWindows_CFoundation_CSize;

#ifndef ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CIStorageFolder __x_ABI_CWindows_CStorage_CIStorageFolder;

#endif // ____x_ABI_CWindows_CStorage_CIStorageFolder_FWD_DEFINED__

#ifndef ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
#define ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference;

#endif // ____x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CSystem_CProcessorArchitecture __x_ABI_CWindows_CSystem_CProcessorArchitecture;

#ifndef ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
#define ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__
typedef interface __x_ABI_CWindows_CSystem_CIUser __x_ABI_CWindows_CSystem_CIUser;

#endif // ____x_ABI_CWindows_CSystem_CIUser_FWD_DEFINED__

typedef enum __x_ABI_CWindows_CApplicationModel_CAddResourcePackageOptions __x_ABI_CWindows_CApplicationModel_CAddResourcePackageOptions;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppExecutionContext __x_ABI_CWindows_CApplicationModel_CAppExecutionContext;

typedef enum __x_ABI_CWindows_CApplicationModel_CAppInstallerPolicySource __x_ABI_CWindows_CApplicationModel_CAppInstallerPolicySource;

typedef enum __x_ABI_CWindows_CApplicationModel_CFullTrustLaunchResult __x_ABI_CWindows_CApplicationModel_CFullTrustLaunchResult;

typedef enum __x_ABI_CWindows_CApplicationModel_CLimitedAccessFeatureStatus __x_ABI_CWindows_CApplicationModel_CLimitedAccessFeatureStatus;

typedef enum __x_ABI_CWindows_CApplicationModel_CPackageContentGroupState __x_ABI_CWindows_CApplicationModel_CPackageContentGroupState;

typedef enum __x_ABI_CWindows_CApplicationModel_CPackageRelationship __x_ABI_CWindows_CApplicationModel_CPackageRelationship;

typedef enum __x_ABI_CWindows_CApplicationModel_CPackageSignatureKind __x_ABI_CWindows_CApplicationModel_CPackageSignatureKind;

typedef enum __x_ABI_CWindows_CApplicationModel_CPackageUpdateAvailability __x_ABI_CWindows_CApplicationModel_CPackageUpdateAvailability;

typedef struct __x_ABI_CWindows_CApplicationModel_CPackageVersion __x_ABI_CWindows_CApplicationModel_CPackageVersion;

/*
 *
 * Struct Windows.ApplicationModel.AddResourcePackageOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
enum __x_ABI_CWindows_CApplicationModel_CAddResourcePackageOptions
{
    AddResourcePackageOptions_None = 0,
    AddResourcePackageOptions_ForceTargetAppShutdown = 0x1,
    AddResourcePackageOptions_ApplyUpdateIfAvailable = 0x2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.AppExecutionContext
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
enum __x_ABI_CWindows_CApplicationModel_CAppExecutionContext
{
    AppExecutionContext_Unknown = 0,
    AppExecutionContext_Host = 1,
    AppExecutionContext_Guest = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Struct Windows.ApplicationModel.AppInstallerPolicySource
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
enum __x_ABI_CWindows_CApplicationModel_CAppInstallerPolicySource
{
    AppInstallerPolicySource_Default = 0,
    AppInstallerPolicySource_System = 1,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Struct Windows.ApplicationModel.FullTrustLaunchResult
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
enum __x_ABI_CWindows_CApplicationModel_CFullTrustLaunchResult
{
    FullTrustLaunchResult_Success = 0,
    FullTrustLaunchResult_AccessDenied = 1,
    FullTrustLaunchResult_FileNotFound = 2,
    FullTrustLaunchResult_Unknown = 3,
};
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Struct Windows.ApplicationModel.LimitedAccessFeatureStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CLimitedAccessFeatureStatus
{
    LimitedAccessFeatureStatus_Unavailable = 0,
    LimitedAccessFeatureStatus_Available = 1,
    LimitedAccessFeatureStatus_AvailableWithoutToken = 2,
    LimitedAccessFeatureStatus_Unknown = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.PackageContentGroupState
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
enum __x_ABI_CWindows_CApplicationModel_CPackageContentGroupState
{
    PackageContentGroupState_NotStaged = 0,
    PackageContentGroupState_Queued = 1,
    PackageContentGroupState_Staging = 2,
    PackageContentGroupState_Staged = 3,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Struct Windows.ApplicationModel.PackageRelationship
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
enum __x_ABI_CWindows_CApplicationModel_CPackageRelationship
{
    PackageRelationship_Dependencies = 0,
    PackageRelationship_Dependents = 1,
    PackageRelationship_All = 2,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Struct Windows.ApplicationModel.PackageSignatureKind
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
enum __x_ABI_CWindows_CApplicationModel_CPackageSignatureKind
{
    PackageSignatureKind_None = 0,
    PackageSignatureKind_Developer = 1,
    PackageSignatureKind_Enterprise = 2,
    PackageSignatureKind_Store = 3,
    PackageSignatureKind_System = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Struct Windows.ApplicationModel.PackageUpdateAvailability
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
enum __x_ABI_CWindows_CApplicationModel_CPackageUpdateAvailability
{
    PackageUpdateAvailability_Unknown = 0,
    PackageUpdateAvailability_NoUpdates = 1,
    PackageUpdateAvailability_Available = 2,
    PackageUpdateAvailability_Required = 3,
    PackageUpdateAvailability_Error = 4,
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Struct Windows.ApplicationModel.StartupTaskState
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
enum __x_ABI_CWindows_CApplicationModel_CStartupTaskState
{
    StartupTaskState_Disabled = 0,
    StartupTaskState_DisabledByUser = 1,
    StartupTaskState_Enabled = 2,
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x20000
    StartupTaskState_DisabledByPolicy = 3,
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x20000
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x30000
    StartupTaskState_EnabledByPolicy = 4,
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x30000
};
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Struct Windows.ApplicationModel.PackageInstallProgress
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
struct __x_ABI_CWindows_CApplicationModel_CPackageInstallProgress
{
    UINT32 PercentComplete;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Struct Windows.ApplicationModel.PackageVersion
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
struct __x_ABI_CWindows_CApplicationModel_CPackageVersion
{
    UINT16 Major;
    UINT16 Minor;
    UINT16 Build;
    UINT16 Revision;
};
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IAppDisplayInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppDisplayInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppDisplayInfo[] = L"Windows.ApplicationModel.IAppDisplayInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetLogo)(__x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo* This,
        struct __x_ABI_CWindows_CFoundation_CSize size,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_GetLogo(This, size, value) \
    ((This)->lpVtbl->GetLogo(This, size, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo[] = L"Windows.ApplicationModel.IAppInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_AppUserModelId)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_DisplayInfo)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        __x_ABI_CWindows_CApplicationModel_CIAppDisplayInfo** value);
    HRESULT (STDMETHODCALLTYPE* get_PackageFamilyName)(__x_ABI_CWindows_CApplicationModel_CIAppInfo* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_get_AppUserModelId(This, value) \
    ((This)->lpVtbl->get_AppUserModelId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_get_DisplayInfo(This, value) \
    ((This)->lpVtbl->get_DisplayInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo_get_PackageFamilyName(This, value) \
    ((This)->lpVtbl->get_PackageFamilyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo2[] = L"Windows.ApplicationModel.IAppInfo2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIAppInfo2* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInfo2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo2_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo3[] = L"Windows.ApplicationModel.IAppInfo3";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInfo3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ExecutionContext)(__x_ABI_CWindows_CApplicationModel_CIAppInfo3* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppExecutionContext* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInfo3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInfo3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInfo3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo3_get_ExecutionContext(This, value) \
    ((This)->lpVtbl->get_ExecutionContext(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfo4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 11.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfo4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfo4[] = L"Windows.ApplicationModel.IAppInfo4";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInfo4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SupportedFileExtensions)(__x_ABI_CWindows_CApplicationModel_CIAppInfo4* This,
        UINT32* valueLength,
        HSTRING** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInfo4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInfo4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInfo4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfo4_get_SupportedFileExtensions(This, valueLength, value) \
    ((This)->lpVtbl->get_SupportedFileExtensions(This, valueLength, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfo4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfo4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xb0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInfoStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInfoStatics[] = L"Windows.ApplicationModel.IAppInfoStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInfoStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** value);
    HRESULT (STDMETHODCALLTYPE* GetFromAppUserModelId)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        HSTRING appUserModelId,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** result);
    HRESULT (STDMETHODCALLTYPE* GetFromAppUserModelIdForUser)(__x_ABI_CWindows_CApplicationModel_CIAppInfoStatics* This,
        __x_ABI_CWindows_CSystem_CIUser* user,
        HSTRING appUserModelId,
        __x_ABI_CWindows_CApplicationModel_CIAppInfo** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInfoStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInfoStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_GetFromAppUserModelId(This, appUserModelId, result) \
    ((This)->lpVtbl->GetFromAppUserModelId(This, appUserModelId, result))

#define __x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_GetFromAppUserModelIdForUser(This, user, appUserModelId, result) \
    ((This)->lpVtbl->GetFromAppUserModelIdForUser(This, user, appUserModelId, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInfoStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInfoStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstallerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstallerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstallerInfo[] = L"Windows.ApplicationModel.IAppInstallerInfo";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfoVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Uri)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfoVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfoVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_get_Uri(This, value) \
    ((This)->lpVtbl->get_Uri(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstallerInfo2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 13.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstallerInfo
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstallerInfo2[] = L"Windows.ApplicationModel.IAppInstallerInfo2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_OnLaunch)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_HoursBetweenUpdateChecks)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        UINT32* value);
    HRESULT (STDMETHODCALLTYPE* get_ShowPrompt)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateBlocksActivation)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_AutomaticBackgroundTask)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ForceUpdateFromAnyVersion)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsAutoRepairEnabled)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        struct __x_ABI_CWindows_CApplicationModel_CPackageVersion* value);
    HRESULT (STDMETHODCALLTYPE* get_LastChecked)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* get_PausedUntil)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);
    HRESULT (STDMETHODCALLTYPE* get_UpdateUris)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_RepairUris)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_DependencyPackageUris)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_OptionalPackageUris)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        __FIVectorView_1_Windows__CFoundation__CUri** value);
    HRESULT (STDMETHODCALLTYPE* get_PolicySource)(__x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2* This,
        enum __x_ABI_CWindows_CApplicationModel_CAppInstallerPolicySource* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_OnLaunch(This, value) \
    ((This)->lpVtbl->get_OnLaunch(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_HoursBetweenUpdateChecks(This, value) \
    ((This)->lpVtbl->get_HoursBetweenUpdateChecks(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_ShowPrompt(This, value) \
    ((This)->lpVtbl->get_ShowPrompt(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_UpdateBlocksActivation(This, value) \
    ((This)->lpVtbl->get_UpdateBlocksActivation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_AutomaticBackgroundTask(This, value) \
    ((This)->lpVtbl->get_AutomaticBackgroundTask(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_ForceUpdateFromAnyVersion(This, value) \
    ((This)->lpVtbl->get_ForceUpdateFromAnyVersion(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_IsAutoRepairEnabled(This, value) \
    ((This)->lpVtbl->get_IsAutoRepairEnabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_LastChecked(This, value) \
    ((This)->lpVtbl->get_LastChecked(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_PausedUntil(This, value) \
    ((This)->lpVtbl->get_PausedUntil(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_UpdateUris(This, value) \
    ((This)->lpVtbl->get_UpdateUris(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_RepairUris(This, value) \
    ((This)->lpVtbl->get_RepairUris(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_DependencyPackageUris(This, value) \
    ((This)->lpVtbl->get_DependencyPackageUris(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_OptionalPackageUris(This, value) \
    ((This)->lpVtbl->get_OptionalPackageUris(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_get_PolicySource(This, value) \
    ((This)->lpVtbl->get_PolicySource(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xd0000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstance_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstance_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstance[] = L"Windows.ApplicationModel.IAppInstance";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInstanceVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Key)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsCurrentInstance)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* RedirectActivationTo)(__x_ABI_CWindows_CApplicationModel_CIAppInstance* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInstanceVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInstance
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInstanceVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_get_Key(This, value) \
    ((This)->lpVtbl->get_Key(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_get_IsCurrentInstance(This, value) \
    ((This)->lpVtbl->get_IsCurrentInstance(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstance_RedirectActivationTo(This) \
    ((This)->lpVtbl->RedirectActivationTo(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstance;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstance_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IAppInstanceStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.AppInstance
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IAppInstanceStatics[] = L"Windows.ApplicationModel.IAppInstanceStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIAppInstanceStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RecommendedInstance)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** value);
    HRESULT (STDMETHODCALLTYPE* GetActivatedEventArgs)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        __x_ABI_CWindows_CApplicationModel_CActivation_CIActivatedEventArgs** result);
    HRESULT (STDMETHODCALLTYPE* FindOrRegisterInstanceForKey)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        HSTRING key,
        __x_ABI_CWindows_CApplicationModel_CIAppInstance** result);
    HRESULT (STDMETHODCALLTYPE* Unregister)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetInstances)(__x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics* This,
        __FIVector_1_Windows__CApplicationModel__CAppInstance** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIAppInstanceStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIAppInstanceStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_get_RecommendedInstance(This, value) \
    ((This)->lpVtbl->get_RecommendedInstance(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_GetActivatedEventArgs(This, result) \
    ((This)->lpVtbl->GetActivatedEventArgs(This, result))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_FindOrRegisterInstanceForKey(This, key, result) \
    ((This)->lpVtbl->FindOrRegisterInstanceForKey(This, key, result))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_Unregister(This) \
    ((This)->lpVtbl->Unregister(This))

#define __x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_GetInstances(This, result) \
    ((This)->lpVtbl->GetInstances(This, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIAppInstanceStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IDesignModeStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DesignMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IDesignModeStatics[] = L"Windows.ApplicationModel.IDesignModeStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIDesignModeStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesignModeEnabled)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIDesignModeStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIDesignModeStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_get_DesignModeEnabled(This, value) \
    ((This)->lpVtbl->get_DesignModeEnabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIDesignModeStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IDesignModeStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.DesignMode
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IDesignModeStatics2[] = L"Windows.ApplicationModel.IDesignModeStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DesignMode2Enabled)(__x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_get_DesignMode2Enabled(This, value) \
    ((This)->lpVtbl->get_DesignMode2Enabled(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIDesignModeStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.IEnteredBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IEnteredBackgroundEventArgs[] = L"Windows.ApplicationModel.IEnteredBackgroundEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIEnteredBackgroundEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IFindRelatedPackagesOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FindRelatedPackagesOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFindRelatedPackagesOptions[] = L"Windows.ApplicationModel.IFindRelatedPackagesOptions";
typedef struct __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Relationship)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPackageRelationship* value);
    HRESULT (STDMETHODCALLTYPE* put_Relationship)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        enum __x_ABI_CWindows_CApplicationModel_CPackageRelationship value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeFrameworks)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeFrameworks)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeHostRuntimes)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeHostRuntimes)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeOptionals)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeOptionals)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean value);
    HRESULT (STDMETHODCALLTYPE* get_IncludeResources)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* put_IncludeResources)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* This,
        boolean value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_get_Relationship(This, value) \
    ((This)->lpVtbl->get_Relationship(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_put_Relationship(This, value) \
    ((This)->lpVtbl->put_Relationship(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_get_IncludeFrameworks(This, value) \
    ((This)->lpVtbl->get_IncludeFrameworks(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_put_IncludeFrameworks(This, value) \
    ((This)->lpVtbl->put_IncludeFrameworks(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_get_IncludeHostRuntimes(This, value) \
    ((This)->lpVtbl->get_IncludeHostRuntimes(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_put_IncludeHostRuntimes(This, value) \
    ((This)->lpVtbl->put_IncludeHostRuntimes(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_get_IncludeOptionals(This, value) \
    ((This)->lpVtbl->get_IncludeOptionals(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_put_IncludeOptionals(This, value) \
    ((This)->lpVtbl->put_IncludeOptionals(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_get_IncludeResources(This, value) \
    ((This)->lpVtbl->get_IncludeResources(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_put_IncludeResources(This, value) \
    ((This)->lpVtbl->put_IncludeResources(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IFindRelatedPackagesOptionsFactory
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FindRelatedPackagesOptions
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFindRelatedPackagesOptionsFactory[] = L"Windows.ApplicationModel.IFindRelatedPackagesOptionsFactory";
typedef struct __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactoryVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* CreateInstance)(__x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory* This,
        enum __x_ABI_CWindows_CApplicationModel_CPackageRelationship Relationship,
        __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactoryVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactoryVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_CreateInstance(This, Relationship, value) \
    ((This)->lpVtbl->CreateInstance(This, Relationship, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptionsFactory_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IFullTrustProcessLaunchResult
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FullTrustProcessLaunchResult
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFullTrustProcessLaunchResult[] = L"Windows.ApplicationModel.IFullTrustProcessLaunchResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_LaunchResult)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CFullTrustLaunchResult* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_get_LaunchResult(This, value) \
    ((This)->lpVtbl->get_LaunchResult(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLaunchResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.IFullTrustProcessLauncherStatics
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FullTrustProcessLauncher
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFullTrustProcessLauncherStatics[] = L"Windows.ApplicationModel.IFullTrustProcessLauncherStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LaunchFullTrustProcessForCurrentAppAsync)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* LaunchFullTrustProcessForCurrentAppWithParametersAsync)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        HSTRING parameterGroupId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* LaunchFullTrustProcessForAppAsync)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        HSTRING fullTrustPackageRelativeAppId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);
    HRESULT (STDMETHODCALLTYPE* LaunchFullTrustProcessForAppWithParametersAsync)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics* This,
        HSTRING fullTrustPackageRelativeAppId,
        HSTRING parameterGroupId,
        __x_ABI_CWindows_CFoundation_CIAsyncAction** asyncAction);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_LaunchFullTrustProcessForCurrentAppAsync(This, asyncAction) \
    ((This)->lpVtbl->LaunchFullTrustProcessForCurrentAppAsync(This, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_LaunchFullTrustProcessForCurrentAppWithParametersAsync(This, parameterGroupId, asyncAction) \
    ((This)->lpVtbl->LaunchFullTrustProcessForCurrentAppWithParametersAsync(This, parameterGroupId, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_LaunchFullTrustProcessForAppAsync(This, fullTrustPackageRelativeAppId, asyncAction) \
    ((This)->lpVtbl->LaunchFullTrustProcessForAppAsync(This, fullTrustPackageRelativeAppId, asyncAction))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_LaunchFullTrustProcessForAppWithParametersAsync(This, fullTrustPackageRelativeAppId, parameterGroupId, asyncAction) \
    ((This)->lpVtbl->LaunchFullTrustProcessForAppWithParametersAsync(This, fullTrustPackageRelativeAppId, parameterGroupId, asyncAction))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IFullTrustProcessLauncherStatics2
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.FullTrustProcessLauncher
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IFullTrustProcessLauncherStatics2[] = L"Windows.ApplicationModel.IFullTrustProcessLauncherStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* LaunchFullTrustProcessForCurrentAppWithArgumentsAsync)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This,
        HSTRING commandLine,
        __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult** operation);
    HRESULT (STDMETHODCALLTYPE* LaunchFullTrustProcessForAppWithArgumentsAsync)(__x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2* This,
        HSTRING fullTrustPackageRelativeAppId,
        HSTRING commandLine,
        __FIAsyncOperation_1_Windows__CApplicationModel__CFullTrustProcessLaunchResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(This, commandLine, operation) \
    ((This)->lpVtbl->LaunchFullTrustProcessForCurrentAppWithArgumentsAsync(This, commandLine, operation))

#define __x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_LaunchFullTrustProcessForAppWithArgumentsAsync(This, fullTrustPackageRelativeAppId, commandLine, operation) \
    ((This)->lpVtbl->LaunchFullTrustProcessForAppWithArgumentsAsync(This, fullTrustPackageRelativeAppId, commandLine, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIFullTrustProcessLauncherStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Interface Windows.ApplicationModel.ILeavingBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ILeavingBackgroundEventArgs[] = L"Windows.ApplicationModel.ILeavingBackgroundEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs* This,
        __x_ABI_CWindows_CFoundation_CIDeferral** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_GetDeferral(This, value) \
    ((This)->lpVtbl->GetDeferral(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CILeavingBackgroundEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.ILimitedAccessFeatureRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LimitedAccessFeatureRequestResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ILimitedAccessFeatureRequestResult[] = L"Windows.ApplicationModel.ILimitedAccessFeatureRequestResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_FeatureId)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CLimitedAccessFeatureStatus* value);
    HRESULT (STDMETHODCALLTYPE* get_EstimatedRemovalDate)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult* This,
        __FIReference_1_Windows__CFoundation__CDateTime** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_get_FeatureId(This, value) \
    ((This)->lpVtbl->get_FeatureId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_get_EstimatedRemovalDate(This, value) \
    ((This)->lpVtbl->get_EstimatedRemovalDate(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.ILimitedAccessFeaturesStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.LimitedAccessFeatures
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ILimitedAccessFeaturesStatics[] = L"Windows.ApplicationModel.ILimitedAccessFeaturesStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* TryUnlockFeature)(__x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics* This,
        HSTRING featureId,
        HSTRING token,
        HSTRING attestation,
        __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeatureRequestResult** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_TryUnlockFeature(This, featureId, token, attestation, result) \
    ((This)->lpVtbl->TryUnlockFeature(This, featureId, token, attestation, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CILimitedAccessFeaturesStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IPackage
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage[] = L"Windows.ApplicationModel.IPackage";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Id)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageId** value);
    HRESULT (STDMETHODCALLTYPE* get_InstalledLocation)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_IsFramework)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Dependencies)(__x_ABI_CWindows_CApplicationModel_CIPackage* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_get_Id(This, value) \
    ((This)->lpVtbl->get_Id(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_get_InstalledLocation(This, value) \
    ((This)->lpVtbl->get_InstalledLocation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_get_IsFramework(This, value) \
    ((This)->lpVtbl->get_IsFramework(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage_get_Dependencies(This, value) \
    ((This)->lpVtbl->get_Dependencies(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackage2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage2[] = L"Windows.ApplicationModel.IPackage2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_DisplayName)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublisherDisplayName)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Description)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Logo)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        __x_ABI_CWindows_CFoundation_CIUriRuntimeClass** value);
    HRESULT (STDMETHODCALLTYPE* get_IsResourcePackage)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsBundle)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_IsDevelopmentMode)(__x_ABI_CWindows_CApplicationModel_CIPackage2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_DisplayName(This, value) \
    ((This)->lpVtbl->get_DisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_PublisherDisplayName(This, value) \
    ((This)->lpVtbl->get_PublisherDisplayName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_Description(This, value) \
    ((This)->lpVtbl->get_Description(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_Logo(This, value) \
    ((This)->lpVtbl->get_Logo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_IsResourcePackage(This, value) \
    ((This)->lpVtbl->get_IsResourcePackage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_IsBundle(This, value) \
    ((This)->lpVtbl->get_IsBundle(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage2_get_IsDevelopmentMode(This, value) \
    ((This)->lpVtbl->get_IsDevelopmentMode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackage3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage3[] = L"Windows.ApplicationModel.IPackage3";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Status)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageStatus** value);
    HRESULT (STDMETHODCALLTYPE* get_InstalledDate)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* GetAppListEntriesAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage3* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_get_Status(This, value) \
    ((This)->lpVtbl->get_Status(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_get_InstalledDate(This, value) \
    ((This)->lpVtbl->get_InstalledDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage3_GetAppListEntriesAsync(This, operation) \
    ((This)->lpVtbl->GetAppListEntriesAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackage4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage4[] = L"Windows.ApplicationModel.IPackage4";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SignatureKind)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        enum __x_ABI_CWindows_CApplicationModel_CPackageSignatureKind* value);
    HRESULT (STDMETHODCALLTYPE* get_IsOptional)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* VerifyContentIntegrityAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage4* This,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_get_SignatureKind(This, value) \
    ((This)->lpVtbl->get_SignatureKind(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_get_IsOptional(This, value) \
    ((This)->lpVtbl->get_IsOptional(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage4_VerifyContentIntegrityAsync(This, operation) \
    ((This)->lpVtbl->VerifyContentIntegrityAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackage5
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage5_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage5_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage5[] = L"Windows.ApplicationModel.IPackage5";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage5Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetContentGroupsAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** operation);
    HRESULT (STDMETHODCALLTYPE* GetContentGroupAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        HSTRING name,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageContentGroup** operation);
    HRESULT (STDMETHODCALLTYPE* StageContentGroupsAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        __FIIterable_1_HSTRING* names,
        __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** operation);
    HRESULT (STDMETHODCALLTYPE* StageContentGroupsWithPriorityAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        __FIIterable_1_HSTRING* names,
        boolean moveToHeadOfQueue,
        __FIAsyncOperation_1___FIVector_1_Windows__CApplicationModel__CPackageContentGroup** operation);
    HRESULT (STDMETHODCALLTYPE* SetInUseAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage5* This,
        boolean inUse,
        __FIAsyncOperation_1_boolean** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage5Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage5
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage5Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_GetContentGroupsAsync(This, operation) \
    ((This)->lpVtbl->GetContentGroupsAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_GetContentGroupAsync(This, name, operation) \
    ((This)->lpVtbl->GetContentGroupAsync(This, name, operation))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_StageContentGroupsAsync(This, names, operation) \
    ((This)->lpVtbl->StageContentGroupsAsync(This, names, operation))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_StageContentGroupsWithPriorityAsync(This, names, moveToHeadOfQueue, operation) \
    ((This)->lpVtbl->StageContentGroupsWithPriorityAsync(This, names, moveToHeadOfQueue, operation))

#define __x_ABI_CWindows_CApplicationModel_CIPackage5_SetInUseAsync(This, inUse, operation) \
    ((This)->lpVtbl->SetInUseAsync(This, inUse, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage5;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage5_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackage6
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage6_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage6_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage6[] = L"Windows.ApplicationModel.IPackage6";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage6Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetAppInstallerInfo)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This,
        __x_ABI_CWindows_CApplicationModel_CIAppInstallerInfo** value);
    HRESULT (STDMETHODCALLTYPE* CheckUpdateAvailabilityAsync)(__x_ABI_CWindows_CApplicationModel_CIPackage6* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageUpdateAvailabilityResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage6Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage6
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage6Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_GetAppInstallerInfo(This, value) \
    ((This)->lpVtbl->GetAppInstallerInfo(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage6_CheckUpdateAvailabilityAsync(This, operation) \
    ((This)->lpVtbl->CheckUpdateAvailabilityAsync(This, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage6;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage6_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IPackage7
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 8.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage7_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage7_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage7[] = L"Windows.ApplicationModel.IPackage7";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage7Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_MutableLocation)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_EffectiveLocation)(__x_ABI_CWindows_CApplicationModel_CIPackage7* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage7Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage7
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage7Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_get_MutableLocation(This, value) \
    ((This)->lpVtbl->get_MutableLocation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage7_get_EffectiveLocation(This, value) \
    ((This)->lpVtbl->get_EffectiveLocation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage7;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage7_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x80000

/*
 *
 * Interface Windows.ApplicationModel.IPackage8
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 10.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage8_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage8_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage8[] = L"Windows.ApplicationModel.IPackage8";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage8Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_EffectiveExternalLocation)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_MachineExternalLocation)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_UserExternalLocation)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        __x_ABI_CWindows_CStorage_CIStorageFolder** value);
    HRESULT (STDMETHODCALLTYPE* get_InstalledPath)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MutablePath)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EffectivePath)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_EffectiveExternalPath)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_MachineExternalPath)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_UserExternalPath)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* GetLogoAsRandomAccessStreamReference)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        struct __x_ABI_CWindows_CFoundation_CSize size,
        __x_ABI_CWindows_CStorage_CStreams_CIRandomAccessStreamReference** result);
    HRESULT (STDMETHODCALLTYPE* GetAppListEntries)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        __FIVectorView_1_Windows__CApplicationModel__CCore__CAppListEntry** result);
    HRESULT (STDMETHODCALLTYPE* get_IsStub)(__x_ABI_CWindows_CApplicationModel_CIPackage8* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage8Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage8
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage8Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_EffectiveExternalLocation(This, value) \
    ((This)->lpVtbl->get_EffectiveExternalLocation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_MachineExternalLocation(This, value) \
    ((This)->lpVtbl->get_MachineExternalLocation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_UserExternalLocation(This, value) \
    ((This)->lpVtbl->get_UserExternalLocation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_InstalledPath(This, value) \
    ((This)->lpVtbl->get_InstalledPath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_MutablePath(This, value) \
    ((This)->lpVtbl->get_MutablePath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_EffectivePath(This, value) \
    ((This)->lpVtbl->get_EffectivePath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_EffectiveExternalPath(This, value) \
    ((This)->lpVtbl->get_EffectiveExternalPath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_MachineExternalPath(This, value) \
    ((This)->lpVtbl->get_MachineExternalPath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_UserExternalPath(This, value) \
    ((This)->lpVtbl->get_UserExternalPath(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_GetLogoAsRandomAccessStreamReference(This, size, result) \
    ((This)->lpVtbl->GetLogoAsRandomAccessStreamReference(This, size, result))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_GetAppListEntries(This, result) \
    ((This)->lpVtbl->GetAppListEntries(This, result))

#define __x_ABI_CWindows_CApplicationModel_CIPackage8_get_IsStub(This, value) \
    ((This)->lpVtbl->get_IsStub(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage8;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage8_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xa0000

/*
 *
 * Interface Windows.ApplicationModel.IPackage9
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackage9_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackage9_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackage9[] = L"Windows.ApplicationModel.IPackage9";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackage9Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* FindRelatedPackages)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This,
        __x_ABI_CWindows_CApplicationModel_CIFindRelatedPackagesOptions* options,
        __FIVector_1_Windows__CApplicationModel__CPackage** result);
    HRESULT (STDMETHODCALLTYPE* get_SourceUriSchemeName)(__x_ABI_CWindows_CApplicationModel_CIPackage9* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackage9Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackage9
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackage9Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_FindRelatedPackages(This, options, result) \
    ((This)->lpVtbl->FindRelatedPackages(This, options, result))

#define __x_ABI_CWindows_CApplicationModel_CIPackage9_get_SourceUriSchemeName(This, value) \
    ((This)->lpVtbl->get_SourceUriSchemeName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackage9;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackage9_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog[] = L"Windows.ApplicationModel.IPackageCatalog";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PackageStaging)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStagingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageStaging)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageInstalling)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageInstallingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageInstalling)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUpdating)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUpdatingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUpdating)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageUninstalling)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageUninstallingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageUninstalling)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* add_PackageStatusChanged)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageStatusChangedEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageStatusChanged)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog* This,
        EventRegistrationToken token);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_add_PackageStaging(This, handler, token) \
    ((This)->lpVtbl->add_PackageStaging(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_remove_PackageStaging(This, token) \
    ((This)->lpVtbl->remove_PackageStaging(This, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_add_PackageInstalling(This, handler, token) \
    ((This)->lpVtbl->add_PackageInstalling(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_remove_PackageInstalling(This, token) \
    ((This)->lpVtbl->remove_PackageInstalling(This, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_add_PackageUpdating(This, handler, token) \
    ((This)->lpVtbl->add_PackageUpdating(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_remove_PackageUpdating(This, token) \
    ((This)->lpVtbl->remove_PackageUpdating(This, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_add_PackageUninstalling(This, handler, token) \
    ((This)->lpVtbl->add_PackageUninstalling(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_remove_PackageUninstalling(This, token) \
    ((This)->lpVtbl->remove_PackageUninstalling(This, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_add_PackageStatusChanged(This, handler, token) \
    ((This)->lpVtbl->add_PackageStatusChanged(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog_remove_PackageStatusChanged(This, token) \
    ((This)->lpVtbl->remove_PackageStatusChanged(This, token))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog2[] = L"Windows.ApplicationModel.IPackageCatalog2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* add_PackageContentGroupStaging)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        __FITypedEventHandler_2_Windows__CApplicationModel__CPackageCatalog_Windows__CApplicationModel__CPackageContentGroupStagingEventArgs* handler,
        EventRegistrationToken* token);
    HRESULT (STDMETHODCALLTYPE* remove_PackageContentGroupStaging)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        EventRegistrationToken token);
    HRESULT (STDMETHODCALLTYPE* AddOptionalPackageAsync)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog2* This,
        HSTRING optionalPackageFamilyName,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogAddOptionalPackageResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_add_PackageContentGroupStaging(This, handler, token) \
    ((This)->lpVtbl->add_PackageContentGroupStaging(This, handler, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_remove_PackageContentGroupStaging(This, token) \
    ((This)->lpVtbl->remove_PackageContentGroupStaging(This, token))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_AddOptionalPackageAsync(This, optionalPackageFamilyName, operation) \
    ((This)->lpVtbl->AddOptionalPackageAsync(This, optionalPackageFamilyName, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog3
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog3[] = L"Windows.ApplicationModel.IPackageCatalog3";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RemoveOptionalPackagesAsync)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog3* This,
        __FIIterable_1_HSTRING* optionalPackageFamilyNames,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveOptionalPackagesResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_RemoveOptionalPackagesAsync(This, optionalPackageFamilyNames, operation) \
    ((This)->lpVtbl->RemoveOptionalPackagesAsync(This, optionalPackageFamilyNames, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog3;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog3_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalog4
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalog4[] = L"Windows.ApplicationModel.IPackageCatalog4";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* AddResourcePackageAsync)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This,
        HSTRING resourcePackageFamilyName,
        HSTRING resourceID,
        enum __x_ABI_CWindows_CApplicationModel_CAddResourcePackageOptions options,
        __FIAsyncOperationWithProgress_2_Windows__CApplicationModel__CPackageCatalogAddResourcePackageResult_Windows__CApplicationModel__CPackageInstallProgress** operation);
    HRESULT (STDMETHODCALLTYPE* RemoveResourcePackagesAsync)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalog4* This,
        __FIIterable_1_Windows__CApplicationModel__CPackage* resourcePackages,
        __FIAsyncOperation_1_Windows__CApplicationModel__CPackageCatalogRemoveResourcePackagesResult** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_AddResourcePackageAsync(This, resourcePackageFamilyName, resourceID, options, operation) \
    ((This)->lpVtbl->AddResourcePackageAsync(This, resourcePackageFamilyName, resourceID, options, operation))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_RemoveResourcePackagesAsync(This, resourcePackages, operation) \
    ((This)->lpVtbl->RemoveResourcePackagesAsync(This, resourcePackages, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalog4;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalog4_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogAddOptionalPackageResult[] = L"Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddOptionalPackageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogAddResourcePackageResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogAddResourcePackageResult[] = L"Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogAddResourcePackageResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogRemoveOptionalPackagesResult[] = L"Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackagesRemoved)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_get_PackagesRemoved(This, value) \
    ((This)->lpVtbl->get_PackagesRemoved(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveOptionalPackagesResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogRemoveResourcePackagesResult[] = L"Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_PackagesRemoved)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This,
        __FIVectorView_1_Windows__CApplicationModel__CPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_get_PackagesRemoved(This, value) \
    ((This)->lpVtbl->get_PackagesRemoved(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogRemoveResourcePackagesResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageCatalog
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogStatics[] = L"Windows.ApplicationModel.IPackageCatalogStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenForCurrentPackage)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog** value);
    HRESULT (STDMETHODCALLTYPE* OpenForCurrentUser)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics* This,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_OpenForCurrentPackage(This, value) \
    ((This)->lpVtbl->OpenForCurrentPackage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_OpenForCurrentUser(This, value) \
    ((This)->lpVtbl->OpenForCurrentUser(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageCatalogStatics2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageCatalogStatics2[] = L"Windows.ApplicationModel.IPackageCatalogStatics2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* OpenForPackage)(__x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage* package,
        __x_ABI_CWindows_CApplicationModel_CIPackageCatalog** result);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_OpenForPackage(This, package, result) \
    ((This)->lpVtbl->OpenForPackage(This, package, result))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageCatalogStatics2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Interface Windows.ApplicationModel.IPackageContentGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageContentGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageContentGroup[] = L"Windows.ApplicationModel.IPackageContentGroup";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        enum __x_ABI_CWindows_CApplicationModel_CPackageContentGroupState* value);
    HRESULT (STDMETHODCALLTYPE* get_IsRequired)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroup* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_get_IsRequired(This, value) \
    ((This)->lpVtbl->get_IsRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageContentGroup;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroup_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageContentGroupStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageContentGroupStagingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageContentGroupStagingEventArgs[] = L"Windows.ApplicationModel.IPackageContentGroupStagingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        HRESULT* value);
    HRESULT (STDMETHODCALLTYPE* get_ContentGroupName)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_IsContentGroupRequired)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_ContentGroupName(This, value) \
    ((This)->lpVtbl->get_ContentGroupName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_get_IsContentGroupRequired(This, value) \
    ((This)->lpVtbl->get_IsContentGroupRequired(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStagingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageContentGroupStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageContentGroup
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageContentGroupStatics[] = L"Windows.ApplicationModel.IPackageContentGroupStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_RequiredGroupName)(__x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_get_RequiredGroupName(This, value) \
    ((This)->lpVtbl->get_RequiredGroupName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageContentGroupStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageId_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageId_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageId[] = L"Windows.ApplicationModel.IPackageId";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageIdVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Name)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Version)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        struct __x_ABI_CWindows_CApplicationModel_CPackageVersion* value);
    HRESULT (STDMETHODCALLTYPE* get_Architecture)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        enum __x_ABI_CWindows_CSystem_CProcessorArchitecture* value);
    HRESULT (STDMETHODCALLTYPE* get_ResourceId)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Publisher)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_PublisherId)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FullName)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_FamilyName)(__x_ABI_CWindows_CApplicationModel_CIPackageId* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageIdVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageId
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageIdVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_Name(This, value) \
    ((This)->lpVtbl->get_Name(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_Version(This, value) \
    ((This)->lpVtbl->get_Version(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_Architecture(This, value) \
    ((This)->lpVtbl->get_Architecture(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_ResourceId(This, value) \
    ((This)->lpVtbl->get_ResourceId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_Publisher(This, value) \
    ((This)->lpVtbl->get_Publisher(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_PublisherId(This, value) \
    ((This)->lpVtbl->get_PublisherId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_FullName(This, value) \
    ((This)->lpVtbl->get_FullName(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageId_get_FamilyName(This, value) \
    ((This)->lpVtbl->get_FamilyName(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageId;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageId_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageIdWithMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageId
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageIdWithMetadata[] = L"Windows.ApplicationModel.IPackageIdWithMetadata";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ProductId)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This,
        HSTRING* value);
    HRESULT (STDMETHODCALLTYPE* get_Author)(__x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadataVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_get_ProductId(This, value) \
    ((This)->lpVtbl->get_ProductId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_get_Author(This, value) \
    ((This)->lpVtbl->get_Author(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageIdWithMetadata_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageInstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageInstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageInstallingEventArgs[] = L"Windows.ApplicationModel.IPackageInstallingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageInstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStagingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStagingEventArgs[] = L"Windows.ApplicationModel.IPackageStagingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStagingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatics
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatics[] = L"Windows.ApplicationModel.IPackageStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Current)(__x_ABI_CWindows_CApplicationModel_CIPackageStatics* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatics_get_Current(This, value) \
    ((This)->lpVtbl->get_Current(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatus[] = L"Windows.ApplicationModel.IPackageStatus";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageStatusVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* VerifyIsOK)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_NotAvailable)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_PackageOffline)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DataOffline)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Disabled)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_NeedsRemediation)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_LicenseIssue)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Modified)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Tampered)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DependencyIssue)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_Servicing)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_DeploymentInProgress)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageStatusVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageStatus
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageStatusVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_VerifyIsOK(This, value) \
    ((This)->lpVtbl->VerifyIsOK(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_NotAvailable(This, value) \
    ((This)->lpVtbl->get_NotAvailable(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_PackageOffline(This, value) \
    ((This)->lpVtbl->get_PackageOffline(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_DataOffline(This, value) \
    ((This)->lpVtbl->get_DataOffline(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_Disabled(This, value) \
    ((This)->lpVtbl->get_Disabled(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_NeedsRemediation(This, value) \
    ((This)->lpVtbl->get_NeedsRemediation(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_LicenseIssue(This, value) \
    ((This)->lpVtbl->get_LicenseIssue(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_Modified(This, value) \
    ((This)->lpVtbl->get_Modified(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_Tampered(This, value) \
    ((This)->lpVtbl->get_Tampered(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_DependencyIssue(This, value) \
    ((This)->lpVtbl->get_DependencyIssue(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_Servicing(This, value) \
    ((This)->lpVtbl->get_Servicing(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus_get_DeploymentInProgress(This, value) \
    ((This)->lpVtbl->get_DeploymentInProgress(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatus;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatus2
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStatus
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatus2[] = L"Windows.ApplicationModel.IPackageStatus2";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageStatus2Vtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_IsPartiallyStaged)(__x_ABI_CWindows_CApplicationModel_CIPackageStatus2* This,
        boolean* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageStatus2Vtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageStatus2
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageStatus2Vtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatus2_get_IsPartiallyStaged(This, value) \
    ((This)->lpVtbl->get_IsPartiallyStaged(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatus2;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatus2_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Interface Windows.ApplicationModel.IPackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageStatusChangedEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.IPackageStatusChangedEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageStatusChangedEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageUninstallingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageUninstallingEventArgs[] = L"Windows.ApplicationModel.IPackageUninstallingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_Package)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_get_Package(This, value) \
    ((This)->lpVtbl->get_Package(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUninstallingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageUpdateAvailabilityResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageUpdateAvailabilityResult
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageUpdateAvailabilityResult[] = L"Windows.ApplicationModel.IPackageUpdateAvailabilityResult";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResultVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_Availability)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This,
        enum __x_ABI_CWindows_CApplicationModel_CPackageUpdateAvailability* value);
    HRESULT (STDMETHODCALLTYPE* get_ExtendedError)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResultVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResultVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_get_Availability(This, value) \
    ((This)->lpVtbl->get_Availability(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_get_ExtendedError(This, value) \
    ((This)->lpVtbl->get_ExtendedError(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdateAvailabilityResult_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Interface Windows.ApplicationModel.IPackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.PackageUpdatingEventArgs
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageUpdatingEventArgs[] = L"Windows.ApplicationModel.IPackageUpdatingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_ActivityId)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        GUID* value);
    HRESULT (STDMETHODCALLTYPE* get_SourcePackage)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_TargetPackage)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CIPackage** value);
    HRESULT (STDMETHODCALLTYPE* get_Progress)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        DOUBLE* value);
    HRESULT (STDMETHODCALLTYPE* get_IsComplete)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        boolean* value);
    HRESULT (STDMETHODCALLTYPE* get_ErrorCode)(__x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs* This,
        HRESULT* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_get_ActivityId(This, value) \
    ((This)->lpVtbl->get_ActivityId(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_get_SourcePackage(This, value) \
    ((This)->lpVtbl->get_SourcePackage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_get_TargetPackage(This, value) \
    ((This)->lpVtbl->get_TargetPackage(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_get_Progress(This, value) \
    ((This)->lpVtbl->get_Progress(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_get_IsComplete(This, value) \
    ((This)->lpVtbl->get_IsComplete(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_get_ErrorCode(This, value) \
    ((This)->lpVtbl->get_ErrorCode(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageUpdatingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Interface Windows.ApplicationModel.IPackageWithMetadata
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.Package
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IPackageWithMetadata[] = L"Windows.ApplicationModel.IPackageWithMetadata";
typedef struct __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadataVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_InstallDate)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);
    HRESULT (STDMETHODCALLTYPE* GetThumbnailToken)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        HSTRING* value);
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Launch may be altered or unavailable for releases after Windows 8.1. Instead, for SmartCardTrigger scenarios use SmartCardTriggerDetails.TryLaunchSelfAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    HRESULT (STDMETHODCALLTYPE* Launch)(__x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata* This,
        HSTRING parameters);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadataVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadataVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_get_InstallDate(This, value) \
    ((This)->lpVtbl->get_InstallDate(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_GetThumbnailToken(This, value) \
    ((This)->lpVtbl->GetThumbnailToken(This, value))

#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
    DEPRECATED("Launch may be altered or unavailable for releases after Windows 8.1. Instead, for SmartCardTrigger scenarios use SmartCardTriggerDetails.TryLaunchSelfAsync")
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#define __x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_Launch(This, parameters) \
    ((This)->lpVtbl->Launch(This, parameters))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIPackageWithMetadata_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IStartupTask
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.StartupTask
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTask_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTask_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IStartupTask[] = L"Windows.ApplicationModel.IStartupTask";
typedef struct __x_ABI_CWindows_CApplicationModel_CIStartupTaskVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* RequestEnableAsync)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTaskState** operation);
    HRESULT (STDMETHODCALLTYPE* Disable)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This);
    HRESULT (STDMETHODCALLTYPE* get_State)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        enum __x_ABI_CWindows_CApplicationModel_CStartupTaskState* value);
    HRESULT (STDMETHODCALLTYPE* get_TaskId)(__x_ABI_CWindows_CApplicationModel_CIStartupTask* This,
        HSTRING* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIStartupTaskVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIStartupTask
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIStartupTaskVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_RequestEnableAsync(This, operation) \
    ((This)->lpVtbl->RequestEnableAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_Disable(This) \
    ((This)->lpVtbl->Disable(This))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_get_State(This, value) \
    ((This)->lpVtbl->get_State(This, value))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTask_get_TaskId(This, value) \
    ((This)->lpVtbl->get_TaskId(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIStartupTask;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTask_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.IStartupTaskStatics
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 * Interface is a part of the implementation of type Windows.ApplicationModel.StartupTask
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_IStartupTaskStatics[] = L"Windows.ApplicationModel.IStartupTaskStatics";
typedef struct __x_ABI_CWindows_CApplicationModel_CIStartupTaskStaticsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetForCurrentPackageAsync)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This,
        __FIAsyncOperation_1___FIVectorView_1_Windows__CApplicationModel__CStartupTask** operation);
    HRESULT (STDMETHODCALLTYPE* GetAsync)(__x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics* This,
        HSTRING taskId,
        __FIAsyncOperation_1_Windows__CApplicationModel__CStartupTask** operation);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CIStartupTaskStaticsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CIStartupTaskStaticsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_GetForCurrentPackageAsync(This, operation) \
    ((This)->lpVtbl->GetForCurrentPackageAsync(This, operation))

#define __x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_GetAsync(This, taskId, operation) \
    ((This)->lpVtbl->GetAsync(This, taskId, operation))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CIStartupTaskStatics_INTERFACE_DEFINED__) */
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ISuspendingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ISuspendingDeferral[] = L"Windows.ApplicationModel.ISuspendingDeferral";
typedef struct __x_ABI_CWindows_CApplicationModel_CISuspendingDeferralVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* Complete)(__x_ABI_CWindows_CApplicationModel_CISuspendingDeferral* This);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CISuspendingDeferralVtbl;

interface __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CISuspendingDeferralVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_Complete(This) \
    ((This)->lpVtbl->Complete(This))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CISuspendingDeferral;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingDeferral_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ISuspendingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ISuspendingEventArgs[] = L"Windows.ApplicationModel.ISuspendingEventArgs";
typedef struct __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgsVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* get_SuspendingOperation)(__x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs* This,
        __x_ABI_CWindows_CApplicationModel_CISuspendingOperation** value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgsVtbl;

interface __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgsVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_get_SuspendingOperation(This, value) \
    ((This)->lpVtbl->get_SuspendingOperation(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingEventArgs_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Interface Windows.ApplicationModel.ISuspendingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#if !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_INTERFACE_DEFINED__)
#define ____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_INTERFACE_DEFINED__
extern const __declspec(selectany) _Null_terminated_ WCHAR InterfaceName_Windows_ApplicationModel_ISuspendingOperation[] = L"Windows.ApplicationModel.ISuspendingOperation";
typedef struct __x_ABI_CWindows_CApplicationModel_CISuspendingOperationVtbl
{
    BEGIN_INTERFACE

    HRESULT (STDMETHODCALLTYPE* QueryInterface)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This,
        REFIID riid,
        void** ppvObject);
    ULONG (STDMETHODCALLTYPE* AddRef)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This);
    ULONG (STDMETHODCALLTYPE* Release)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This);
    HRESULT (STDMETHODCALLTYPE* GetIids)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This,
        ULONG* iidCount,
        IID** iids);
    HRESULT (STDMETHODCALLTYPE* GetRuntimeClassName)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This,
        HSTRING* className);
    HRESULT (STDMETHODCALLTYPE* GetTrustLevel)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This,
        TrustLevel* trustLevel);
    HRESULT (STDMETHODCALLTYPE* GetDeferral)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This,
        __x_ABI_CWindows_CApplicationModel_CISuspendingDeferral** deferral);
    HRESULT (STDMETHODCALLTYPE* get_Deadline)(__x_ABI_CWindows_CApplicationModel_CISuspendingOperation* This,
        struct __x_ABI_CWindows_CFoundation_CDateTime* value);

    END_INTERFACE
} __x_ABI_CWindows_CApplicationModel_CISuspendingOperationVtbl;

interface __x_ABI_CWindows_CApplicationModel_CISuspendingOperation
{
    CONST_VTBL struct __x_ABI_CWindows_CApplicationModel_CISuspendingOperationVtbl* lpVtbl;
};

#ifdef COBJMACROS

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_QueryInterface(This, riid, ppvObject) \
    ((This)->lpVtbl->QueryInterface(This, riid, ppvObject))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_AddRef(This) \
    ((This)->lpVtbl->AddRef(This))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_Release(This) \
    ((This)->lpVtbl->Release(This))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_GetIids(This, iidCount, iids) \
    ((This)->lpVtbl->GetIids(This, iidCount, iids))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_GetRuntimeClassName(This, className) \
    ((This)->lpVtbl->GetRuntimeClassName(This, className))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_GetTrustLevel(This, trustLevel) \
    ((This)->lpVtbl->GetTrustLevel(This, trustLevel))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_GetDeferral(This, deferral) \
    ((This)->lpVtbl->GetDeferral(This, deferral))

#define __x_ABI_CWindows_CApplicationModel_CISuspendingOperation_get_Deadline(This, value) \
    ((This)->lpVtbl->get_Deadline(This, value))

#endif /* COBJMACROS */

EXTERN_C const IID IID___x_ABI_CWindows_CApplicationModel_CISuspendingOperation;
#endif /* !defined(____x_ABI_CWindows_CApplicationModel_CISuspendingOperation_INTERFACE_DEFINED__) */
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppDisplayInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppDisplayInfo ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppDisplayInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppDisplayInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppDisplayInfo[] = L"Windows.ApplicationModel.AppDisplayInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IAppInfoStatics interface starting with version 10.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppInfo ** Default Interface **
 *    Windows.ApplicationModel.IAppInfo2
 *    Windows.ApplicationModel.IAppInfo3
 *    Windows.ApplicationModel.IAppInfo4
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppInfo[] = L"Windows.ApplicationModel.AppInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.AppInstallerInfo
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppInstallerInfo ** Default Interface **
 *    Windows.ApplicationModel.IAppInstallerInfo2
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppInstallerInfo_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppInstallerInfo_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppInstallerInfo[] = L"Windows.ApplicationModel.AppInstallerInfo";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.AppInstance
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IAppInstanceStatics interface starting with version 6.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IAppInstance ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_AppInstance_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_AppInstance_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_AppInstance[] = L"Windows.ApplicationModel.AppInstance";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.DesignMode
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IDesignModeStatics2 interface starting with version 5.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.IDesignModeStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_DesignMode_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_DesignMode_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_DesignMode[] = L"Windows.ApplicationModel.DesignMode";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.EnteredBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IEnteredBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_EnteredBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_EnteredBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_EnteredBackgroundEventArgs[] = L"Windows.ApplicationModel.EnteredBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.FindRelatedPackagesOptions
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 15.0
 *
 * RuntimeClass can be activated.
 *   Type can be activated via the Windows.ApplicationModel.IFindRelatedPackagesOptionsFactory interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IFindRelatedPackagesOptions ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_FindRelatedPackagesOptions_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_FindRelatedPackagesOptions_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_FindRelatedPackagesOptions[] = L"Windows.ApplicationModel.FindRelatedPackagesOptions";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0xf0000

/*
 *
 * Class Windows.ApplicationModel.FullTrustProcessLaunchResult
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 2.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IFullTrustProcessLaunchResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLaunchResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLaunchResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_FullTrustProcessLaunchResult[] = L"Windows.ApplicationModel.FullTrustProcessLaunchResult";
#endif
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x20000

/*
 *
 * Class Windows.ApplicationModel.FullTrustProcessLauncher
 *
 * Introduced to Windows.ApplicationModel.FullTrustAppContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IFullTrustProcessLauncherStatics2 interface starting with version 2.0 of the Windows.ApplicationModel.FullTrustAppContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.IFullTrustProcessLauncherStatics interface starting with version 1.0 of the Windows.ApplicationModel.FullTrustAppContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLauncher_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_FullTrustProcessLauncher_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_FullTrustProcessLauncher[] = L"Windows.ApplicationModel.FullTrustProcessLauncher";
#endif
#endif // WINDOWS_APPLICATIONMODEL_FULLTRUSTAPPCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.LeavingBackgroundEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ILeavingBackgroundEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LeavingBackgroundEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LeavingBackgroundEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LeavingBackgroundEventArgs[] = L"Windows.ApplicationModel.LeavingBackgroundEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.LimitedAccessFeatureRequestResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ILimitedAccessFeatureRequestResult ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatureRequestResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatureRequestResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LimitedAccessFeatureRequestResult[] = L"Windows.ApplicationModel.LimitedAccessFeatureRequestResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.LimitedAccessFeatures
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.ILimitedAccessFeaturesStatics interface starting with version 7.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatures_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_LimitedAccessFeatures_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_LimitedAccessFeatures[] = L"Windows.ApplicationModel.LimitedAccessFeatures";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.Package
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IPackageStatics interface starting with version 1.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackage ** Default Interface **
 *    Windows.ApplicationModel.IPackage2
 *    Windows.ApplicationModel.IPackage3
 *    Windows.ApplicationModel.IPackageWithMetadata
 *    Windows.ApplicationModel.IPackage4
 *    Windows.ApplicationModel.IPackage5
 *    Windows.ApplicationModel.IPackage6
 *    Windows.ApplicationModel.IPackage7
 *    Windows.ApplicationModel.IPackage8
 *    Windows.ApplicationModel.IPackage9
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_Package_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_Package_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_Package[] = L"Windows.ApplicationModel.Package";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalog
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IPackageCatalogStatics interface starting with version 3.0 of the Windows.Foundation.UniversalApiContract API contract
 *   Static Methods exist on the Windows.ApplicationModel.IPackageCatalogStatics2 interface starting with version 15.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalog ** Default Interface **
 *    Windows.ApplicationModel.IPackageCatalog2
 *    Windows.ApplicationModel.IPackageCatalog3
 *    Windows.ApplicationModel.IPackageCatalog4
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalog_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalog_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalog[] = L"Windows.ApplicationModel.PackageCatalog";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogAddOptionalPackageResult ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddOptionalPackageResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddOptionalPackageResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogAddOptionalPackageResult[] = L"Windows.ApplicationModel.PackageCatalogAddOptionalPackageResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogAddResourcePackageResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogAddResourcePackageResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddResourcePackageResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogAddResourcePackageResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogAddResourcePackageResult[] = L"Windows.ApplicationModel.PackageCatalogAddResourcePackageResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 5.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogRemoveOptionalPackagesResult ** Default Interface **
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveOptionalPackagesResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveOptionalPackagesResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogRemoveOptionalPackagesResult[] = L"Windows.ApplicationModel.PackageCatalogRemoveOptionalPackagesResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x50000

/*
 *
 * Class Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 6.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageCatalogRemoveResourcePackagesResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveResourcePackagesResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageCatalogRemoveResourcePackagesResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageCatalogRemoveResourcePackagesResult[] = L"Windows.ApplicationModel.PackageCatalogRemoveResourcePackagesResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x60000

/*
 *
 * Class Windows.ApplicationModel.PackageContentGroup
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IPackageContentGroupStatics interface starting with version 4.0 of the Windows.Foundation.UniversalApiContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageContentGroup ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroup_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroup_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageContentGroup[] = L"Windows.ApplicationModel.PackageContentGroup";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.PackageContentGroupStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 4.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageContentGroupStagingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroupStagingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageContentGroupStagingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageContentGroupStagingEventArgs[] = L"Windows.ApplicationModel.PackageContentGroupStagingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x40000

/*
 *
 * Class Windows.ApplicationModel.PackageId
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageId ** Default Interface **
 *    Windows.ApplicationModel.IPackageIdWithMetadata
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageId_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageId_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageId[] = L"Windows.ApplicationModel.PackageId";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.PackageInstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageInstallingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageInstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageInstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageInstallingEventArgs[] = L"Windows.ApplicationModel.PackageInstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageStagingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageStagingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageStagingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageStagingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageStagingEventArgs[] = L"Windows.ApplicationModel.PackageStagingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageStatus
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageStatus ** Default Interface **
 *    Windows.ApplicationModel.IPackageStatus2
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageStatus_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageStatus_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageStatus[] = L"Windows.ApplicationModel.PackageStatus";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.PackageStatusChangedEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageStatusChangedEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageStatusChangedEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageStatusChangedEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageStatusChangedEventArgs[] = L"Windows.ApplicationModel.PackageStatusChangedEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageUninstallingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageUninstallingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageUninstallingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageUninstallingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageUninstallingEventArgs[] = L"Windows.ApplicationModel.PackageUninstallingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.PackageUpdateAvailabilityResult
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 7.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageUpdateAvailabilityResult ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageUpdateAvailabilityResult_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageUpdateAvailabilityResult_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageUpdateAvailabilityResult[] = L"Windows.ApplicationModel.PackageUpdateAvailabilityResult";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x70000

/*
 *
 * Class Windows.ApplicationModel.PackageUpdatingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 3.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IPackageUpdatingEventArgs ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_PackageUpdatingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_PackageUpdatingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_PackageUpdatingEventArgs[] = L"Windows.ApplicationModel.PackageUpdatingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x30000

/*
 *
 * Class Windows.ApplicationModel.StartupTask
 *
 * Introduced to Windows.ApplicationModel.StartupTaskContract in version 1.0
 *
 * RuntimeClass contains static methods.
 *   Static Methods exist on the Windows.ApplicationModel.IStartupTaskStatics interface starting with version 1.0 of the Windows.ApplicationModel.StartupTaskContract API contract
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.IStartupTask ** Default Interface **
 *
 * Class Threading Model:  Both Single and Multi Threaded Apartment
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_StartupTask_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_StartupTask_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_StartupTask[] = L"Windows.ApplicationModel.StartupTask";
#endif
#endif // WINDOWS_APPLICATIONMODEL_STARTUPTASKCONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SuspendingDeferral
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingDeferral ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SuspendingDeferral_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SuspendingDeferral_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SuspendingDeferral[] = L"Windows.ApplicationModel.SuspendingDeferral";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SuspendingEventArgs
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingEventArgs ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SuspendingEventArgs_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SuspendingEventArgs_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SuspendingEventArgs[] = L"Windows.ApplicationModel.SuspendingEventArgs";
#endif
#endif // WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000

/*
 *
 * Class Windows.ApplicationModel.SuspendingOperation
 *
 * Introduced to Windows.Foundation.UniversalApiContract in version 1.0
 *
 * Class implements the following interfaces:
 *    Windows.ApplicationModel.ISuspendingOperation ** Default Interface **
 *
 * Class Marshaling Behavior:  Agile - Class is agile
 *
 */
#if WINDOWS_FOUNDATION_UNIVERSALAPICONTRACT_VERSION >= 0x10000
#ifndef RUNTIMECLASS_Windows_ApplicationModel_SuspendingOperation_DEFINED
#define RUNTIMECLASS_Windows_ApplicationModel_SuspendingOperation_DEFINED
extern const __declspec(selectany) _Null_terminated_ WCHAR RuntimeClass_Windows_ApplicationModel_SuspendingOperation[] = L"Windows.ApplicationModel.SuspendingOperation";
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
#endif // __windows2Eapplicationmodel_p_h__

#endif // __windows2Eapplicationmodel_h__
